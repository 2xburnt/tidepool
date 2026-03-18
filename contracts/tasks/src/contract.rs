use cosmwasm_std::{
    entry_point, to_json_binary, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Order,
    Response, StdResult, WasmMsg,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Task, TaskConfig, TASKS, TASK_CONFIG};
use tidepool_types::{
    AgentResponse, ReputationExecuteMsg, TaskConfigResponse, TaskResponse, TaskStatus,
    TasksListResponse, XP_TASK_POSTED_COMPLETED,
};

const CONTRACT_NAME: &str = "crates.io:tidepool-tasks";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let rep_addr = deps.api.addr_validate(&msg.reputation_contract)?;
    let config = TaskConfig {
        owner: info.sender.clone(),
        reputation_contract: rep_addr.clone(),
        next_task_id: 1,
    };
    TASK_CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("reputation_contract", rep_addr))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::PostTask {
            title,
            description,
            xp_reward,
            required_badges,
            expires_in_blocks,
        } => execute_post_task(
            deps,
            env,
            info,
            title,
            description,
            xp_reward,
            required_badges,
            expires_in_blocks,
        ),
        ExecuteMsg::ClaimTask { task_id } => execute_claim_task(deps, env, info, task_id),
        ExecuteMsg::CompleteTask { task_id } => execute_complete_task(deps, env, info, task_id),
        ExecuteMsg::ExpireTask { task_id } => execute_expire_task(deps, env, info, task_id),
    }
}

fn execute_post_task(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    title: String,
    description: String,
    xp_reward: u64,
    required_badges: Vec<String>,
    expires_in_blocks: Option<u64>,
) -> Result<Response, ContractError> {
    let mut config = TASK_CONFIG.load(deps.storage)?;
    let task_id = config.next_task_id;
    config.next_task_id += 1;

    let expires_at = expires_in_blocks.map(|blocks| env.block.height + blocks);

    let task = Task {
        id: task_id,
        poster: info.sender.clone(),
        title: title.clone(),
        description,
        xp_reward,
        required_badges,
        status: TaskStatus::Open,
        claimant: None,
        created_at: env.block.height,
        claimed_at: None,
        completed_at: None,
        expires_at,
    };

    TASKS.save(deps.storage, task_id, &task)?;
    TASK_CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("method", "post_task")
        .add_attribute("task_id", task_id.to_string())
        .add_attribute("poster", info.sender)
        .add_attribute("title", title))
}

fn execute_claim_task(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    task_id: u64,
) -> Result<Response, ContractError> {
    let config = TASK_CONFIG.load(deps.storage)?;
    let mut task = TASKS
        .may_load(deps.storage, task_id)?
        .ok_or(ContractError::TaskNotFound {})?;

    if task.status != TaskStatus::Open {
        return Err(ContractError::TaskNotOpen {});
    }

    if task.poster == info.sender {
        return Err(ContractError::CannotClaimOwnTask {});
    }

    // Check expiry
    if let Some(expires_at) = task.expires_at {
        if env.block.height >= expires_at {
            return Err(ContractError::TaskNotOpen {});
        }
    }

    // Verify agent is registered by querying reputation contract
    let agent_resp: AgentResponse = deps.querier.query_wasm_smart(
        &config.reputation_contract,
        &tidepool_types::ReputationQueryMsg::GetAgent {
            address: info.sender.to_string(),
        },
    ).map_err(|_| ContractError::AgentNotRegistered {})?;

    // Check required badges
    for required in &task.required_badges {
        if !agent_resp.badges.iter().any(|b| &b.badge_type == required) {
            return Err(ContractError::MissingBadge {
                badge: required.clone(),
            });
        }
    }

    task.status = TaskStatus::Claimed;
    task.claimant = Some(info.sender.clone());
    task.claimed_at = Some(env.block.height);
    TASKS.save(deps.storage, task_id, &task)?;

    Ok(Response::new()
        .add_attribute("method", "claim_task")
        .add_attribute("task_id", task_id.to_string())
        .add_attribute("claimant", info.sender))
}

fn execute_complete_task(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    task_id: u64,
) -> Result<Response, ContractError> {
    let config = TASK_CONFIG.load(deps.storage)?;
    let mut task = TASKS
        .may_load(deps.storage, task_id)?
        .ok_or(ContractError::TaskNotFound {})?;

    if task.status != TaskStatus::Claimed {
        return Err(ContractError::TaskNotClaimed {});
    }

    if task.poster != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    let claimant = task.claimant.clone().unwrap();

    task.status = TaskStatus::Completed;
    task.completed_at = Some(env.block.height);
    TASKS.save(deps.storage, task_id, &task)?;

    // Cross-contract calls to reputation contract
    let rep_contract = config.reputation_contract.to_string();
    let msgs: Vec<CosmosMsg> = vec![
        // Award XP to claimant
        WasmMsg::Execute {
            contract_addr: rep_contract.clone(),
            msg: to_json_binary(&ReputationExecuteMsg::AwardXp {
                agent: claimant.to_string(),
                amount: task.xp_reward,
                reason: format!("task_completion:{}", task_id),
            })?,
            funds: vec![],
        }
        .into(),
        // Increment tasks completed for claimant
        WasmMsg::Execute {
            contract_addr: rep_contract.clone(),
            msg: to_json_binary(&ReputationExecuteMsg::IncrementTasksCompleted {
                agent: claimant.to_string(),
            })?,
            funds: vec![],
        }
        .into(),
        // Award poster XP
        WasmMsg::Execute {
            contract_addr: rep_contract.clone(),
            msg: to_json_binary(&ReputationExecuteMsg::AwardXp {
                agent: info.sender.to_string(),
                amount: XP_TASK_POSTED_COMPLETED,
                reason: format!("task_posted_completed:{}", task_id),
            })?,
            funds: vec![],
        }
        .into(),
        // Increment tasks posted for poster
        WasmMsg::Execute {
            contract_addr: rep_contract,
            msg: to_json_binary(&ReputationExecuteMsg::IncrementTasksPosted {
                agent: info.sender.to_string(),
            })?,
            funds: vec![],
        }
        .into(),
    ];

    Ok(Response::new()
        .add_messages(msgs)
        .add_attribute("method", "complete_task")
        .add_attribute("task_id", task_id.to_string())
        .add_attribute("claimant", claimant)
        .add_attribute("xp_awarded", task.xp_reward.to_string()))
}

fn execute_expire_task(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    task_id: u64,
) -> Result<Response, ContractError> {
    let mut task = TASKS
        .may_load(deps.storage, task_id)?
        .ok_or(ContractError::TaskNotFound {})?;

    if task.status == TaskStatus::Completed || task.status == TaskStatus::Expired {
        return Err(ContractError::TaskNotOpen {});
    }

    let expires_at = task.expires_at.ok_or(ContractError::TaskNotExpired {})?;
    if env.block.height < expires_at {
        return Err(ContractError::TaskNotExpired {});
    }

    task.status = TaskStatus::Expired;
    task.claimant = None;
    task.claimed_at = None;
    TASKS.save(deps.storage, task_id, &task)?;

    Ok(Response::new()
        .add_attribute("method", "expire_task")
        .add_attribute("task_id", task_id.to_string()))
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetTask { task_id } => to_json_binary(&query_task(deps, task_id)?),
        QueryMsg::ListTasks {
            status,
            start_after,
            limit,
        } => to_json_binary(&query_list_tasks(deps, status, start_after, limit)?),
        QueryMsg::MyPostedTasks { address } => {
            to_json_binary(&query_my_posted_tasks(deps, address)?)
        }
        QueryMsg::MyClaimedTasks { address } => {
            to_json_binary(&query_my_claimed_tasks(deps, address)?)
        }
        QueryMsg::Config {} => to_json_binary(&query_config(deps)?),
    }
}

fn task_to_response(task: &Task) -> TaskResponse {
    TaskResponse {
        id: task.id,
        poster: task.poster.clone(),
        title: task.title.clone(),
        description: task.description.clone(),
        xp_reward: task.xp_reward,
        required_badges: task.required_badges.clone(),
        status: task.status.clone(),
        claimant: task.claimant.clone(),
        created_at: task.created_at,
        claimed_at: task.claimed_at,
        completed_at: task.completed_at,
        expires_at: task.expires_at,
    }
}

fn query_task(deps: Deps, task_id: u64) -> StdResult<TaskResponse> {
    let task = TASKS.load(deps.storage, task_id)?;
    Ok(task_to_response(&task))
}

fn query_list_tasks(
    deps: Deps,
    status: Option<TaskStatus>,
    start_after: Option<u64>,
    limit: Option<u32>,
) -> StdResult<TasksListResponse> {
    let limit = limit.unwrap_or(30).min(100) as usize;
    let start = start_after.map(cw_storage_plus::Bound::exclusive);

    let tasks: Vec<TaskResponse> = TASKS
        .range(deps.storage, start, None, Order::Ascending)
        .filter_map(|item| {
            let (_, task) = item.ok()?;
            if let Some(ref s) = status {
                if &task.status != s {
                    return None;
                }
            }
            Some(task_to_response(&task))
        })
        .take(limit)
        .collect();

    Ok(TasksListResponse { tasks })
}

fn query_my_posted_tasks(deps: Deps, address: String) -> StdResult<TasksListResponse> {
    let addr = deps.api.addr_validate(&address)?;
    let tasks: Vec<TaskResponse> = TASKS
        .range(deps.storage, None, None, Order::Ascending)
        .filter_map(|item| {
            let (_, task) = item.ok()?;
            if task.poster == addr {
                Some(task_to_response(&task))
            } else {
                None
            }
        })
        .collect();
    Ok(TasksListResponse { tasks })
}

fn query_my_claimed_tasks(deps: Deps, address: String) -> StdResult<TasksListResponse> {
    let addr = deps.api.addr_validate(&address)?;
    let tasks: Vec<TaskResponse> = TASKS
        .range(deps.storage, None, None, Order::Ascending)
        .filter_map(|item| {
            let (_, task) = item.ok()?;
            if task.claimant.as_ref() == Some(&addr) {
                Some(task_to_response(&task))
            } else {
                None
            }
        })
        .collect();
    Ok(TasksListResponse { tasks })
}

fn query_config(deps: Deps) -> StdResult<TaskConfigResponse> {
    let config = TASK_CONFIG.load(deps.storage)?;
    Ok(TaskConfigResponse {
        owner: config.owner,
        reputation_contract: config.reputation_contract,
        next_task_id: config.next_task_id,
    })
}
