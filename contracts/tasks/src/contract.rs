use cosmwasm_std::{
    entry_point, to_json_binary, BankMsg, Binary, CosmosMsg, Deps, DepsMut, Env,
    MessageInfo, Order, Response, StdResult, Uint128, Uint256, WasmMsg,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Task, TaskConfig, TASKS, TASK_CONFIG};
use tidepool_types::{
    ReputationExecuteMsg, ReputationQueryMsg, AgentResponse,
    TaskConfigResponse, TaskResponse, TaskStatus, TasksListResponse,
    AUTO_RELEASE_SECONDS, DEFAULT_MINIMUM_ESCROW, ESCROW_DENOM,
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
    let minimum_escrow = msg
        .minimum_escrow
        .unwrap_or(Uint128::new(DEFAULT_MINIMUM_ESCROW));

    let config = TaskConfig {
        owner: info.sender.clone(),
        reputation_contract: rep_addr.clone(),
        next_task_id: 1,
        minimum_escrow,
        denom: ESCROW_DENOM.to_string(),
    };
    TASK_CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("reputation_contract", rep_addr)
        .add_attribute("minimum_escrow", minimum_escrow)
        .add_attribute("denom", ESCROW_DENOM))
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
            category,
            specializations,
            expires_in_blocks,
        } => execute_post_task(deps, env, info, title, description, category, specializations, expires_in_blocks),
        ExecuteMsg::ClaimTask { task_id } => execute_claim_task(deps, env, info, task_id),
        ExecuteMsg::SubmitTask { task_id } => execute_submit_task(deps, env, info, task_id),
        ExecuteMsg::ApproveTask { task_id } => execute_approve_task(deps, env, info, task_id),
        ExecuteMsg::AutoRelease { task_id } => execute_auto_release(deps, env, info, task_id),
        ExecuteMsg::CancelTask { task_id } => execute_cancel_task(deps, env, info, task_id),
        ExecuteMsg::ExpireTask { task_id } => execute_expire_task(deps, env, info, task_id),
        ExecuteMsg::UpdateConfig { minimum_escrow } => {
            execute_update_config(deps, info, minimum_escrow)
        }
    }
}

fn execute_post_task(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    title: String,
    description: String,
    category: Option<String>,
    specializations: Vec<String>,
    expires_in_blocks: Option<u64>,
) -> Result<Response, ContractError> {
    let mut config = TASK_CONFIG.load(deps.storage)?;

    // Validate funds: exactly one coin, correct denom, >= minimum
    if info.funds.len() != 1 {
        return Err(ContractError::InvalidFunds {});
    }
    let sent = &info.funds[0];
    if sent.denom != config.denom {
        return Err(ContractError::WrongDenom {
            expected: config.denom.clone(),
            got: sent.denom.clone(),
        });
    }
    if sent.amount < Uint256::from(config.minimum_escrow) {
        return Err(ContractError::EscrowTooLow {
            sent: sent.amount.to_string(),
            minimum: config.minimum_escrow.to_string(),
        });
    }

    let task_id = config.next_task_id;
    config.next_task_id += 1;

    let expires_at = expires_in_blocks.map(|blocks| env.block.height + blocks);

    let task = Task {
        id: task_id,
        poster: info.sender.clone(),
        title: title.clone(),
        description,
        category: category.clone(),
        specializations: specializations.clone(),
        escrow: sent.clone(),
        status: TaskStatus::Open,
        claimant: None,
        created_at: env.block.height,
        claimed_at: None,
        submitted_at: None,
        completed_at: None,
        expires_at,
    };

    TASKS.save(deps.storage, task_id, &task)?;
    TASK_CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("method", "post_task")
        .add_attribute("task_id", task_id.to_string())
        .add_attribute("poster", info.sender)
        .add_attribute("title", title)
        .add_attribute("escrow", format!("{}{}", sent.amount, sent.denom)))
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
    let _agent_resp: AgentResponse = deps
        .querier
        .query_wasm_smart(
            &config.reputation_contract,
            &ReputationQueryMsg::GetAgent {
                address: info.sender.to_string(),
            },
        )
        .map_err(|_| ContractError::AgentNotRegistered {})?;

    task.status = TaskStatus::Claimed;
    task.claimant = Some(info.sender.clone());
    task.claimed_at = Some(env.block.height);
    TASKS.save(deps.storage, task_id, &task)?;

    Ok(Response::new()
        .add_attribute("method", "claim_task")
        .add_attribute("task_id", task_id.to_string())
        .add_attribute("claimant", info.sender))
}

fn execute_submit_task(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    task_id: u64,
) -> Result<Response, ContractError> {
    let mut task = TASKS
        .may_load(deps.storage, task_id)?
        .ok_or(ContractError::TaskNotFound {})?;

    if task.status != TaskStatus::Claimed {
        return Err(ContractError::TaskNotClaimed {});
    }

    // Only the claimant (worker) can submit
    if task.claimant.as_ref() != Some(&info.sender) {
        return Err(ContractError::Unauthorized {});
    }

    task.status = TaskStatus::Submitted;
    task.submitted_at = Some(env.block.time.seconds());
    TASKS.save(deps.storage, task_id, &task)?;

    Ok(Response::new()
        .add_attribute("method", "submit_task")
        .add_attribute("task_id", task_id.to_string())
        .add_attribute("worker", info.sender))
}

fn settle_task(
    deps: DepsMut,
    env: Env,
    task: &mut Task,
    task_id: u64,
) -> Result<Vec<CosmosMsg>, ContractError> {
    let config = TASK_CONFIG.load(deps.storage)?;
    let claimant = task.claimant.clone().unwrap();

    task.status = TaskStatus::Completed;
    task.completed_at = Some(env.block.height);
    TASKS.save(deps.storage, task_id, task)?;

    let mut msgs: Vec<CosmosMsg> = vec![];

    // Send escrow to worker
    msgs.push(
        BankMsg::Send {
            to_address: claimant.to_string(),
            amount: vec![task.escrow.clone()],
        }
        .into(),
    );

    // Update volume in reputation contract
    msgs.push(
        WasmMsg::Execute {
            contract_addr: config.reputation_contract.to_string(),
            msg: to_json_binary(&ReputationExecuteMsg::UpdateVolume {
                worker: claimant.to_string(),
                poster: task.poster.to_string(),
                amount: Uint128::try_from(task.escrow.amount).unwrap(),
            })?,
            funds: vec![],
        }
        .into(),
    );

    Ok(msgs)
}

fn execute_approve_task(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    task_id: u64,
) -> Result<Response, ContractError> {
    let mut task = TASKS
        .may_load(deps.storage, task_id)?
        .ok_or(ContractError::TaskNotFound {})?;

    if task.status != TaskStatus::Submitted {
        return Err(ContractError::TaskNotSubmitted {});
    }

    // Only the poster can approve
    if task.poster != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    let claimant = task.claimant.clone().unwrap();
    let msgs = settle_task(deps, env, &mut task, task_id)?;

    Ok(Response::new()
        .add_messages(msgs)
        .add_attribute("method", "approve_task")
        .add_attribute("task_id", task_id.to_string())
        .add_attribute("claimant", claimant)
        .add_attribute("escrow_paid", format!("{}{}", task.escrow.amount, task.escrow.denom)))
}

fn execute_auto_release(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    task_id: u64,
) -> Result<Response, ContractError> {
    let mut task = TASKS
        .may_load(deps.storage, task_id)?
        .ok_or(ContractError::TaskNotFound {})?;

    if task.status != TaskStatus::Submitted {
        return Err(ContractError::TaskNotSubmitted {});
    }

    let submitted_at = task.submitted_at.unwrap();
    let now = env.block.time.seconds();

    if now < submitted_at + AUTO_RELEASE_SECONDS {
        return Err(ContractError::AutoReleaseNotReady {
            remaining: (submitted_at + AUTO_RELEASE_SECONDS) - now,
        });
    }

    let claimant = task.claimant.clone().unwrap();
    let msgs = settle_task(deps, env, &mut task, task_id)?;

    Ok(Response::new()
        .add_messages(msgs)
        .add_attribute("method", "auto_release")
        .add_attribute("task_id", task_id.to_string())
        .add_attribute("claimant", claimant)
        .add_attribute("escrow_paid", format!("{}{}", task.escrow.amount, task.escrow.denom)))
}

fn execute_cancel_task(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    task_id: u64,
) -> Result<Response, ContractError> {
    let mut task = TASKS
        .may_load(deps.storage, task_id)?
        .ok_or(ContractError::TaskNotFound {})?;

    // Only poster can cancel, and only if unclaimed
    if task.poster != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    if task.status != TaskStatus::Open {
        return Err(ContractError::CannotCancel {});
    }

    task.status = TaskStatus::Cancelled;
    TASKS.save(deps.storage, task_id, &task)?;

    // Refund escrow to poster
    let refund_msg = BankMsg::Send {
        to_address: task.poster.to_string(),
        amount: vec![task.escrow.clone()],
    };

    Ok(Response::new()
        .add_message(refund_msg)
        .add_attribute("method", "cancel_task")
        .add_attribute("task_id", task_id.to_string())
        .add_attribute("escrow_refunded", format!("{}{}", task.escrow.amount, task.escrow.denom)))
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

    // Can only expire open (unclaimed) tasks
    if task.status != TaskStatus::Open {
        return Err(ContractError::TaskNotOpen {});
    }

    let expires_at = task.expires_at.ok_or(ContractError::TaskNotExpired {})?;
    if env.block.height < expires_at {
        return Err(ContractError::TaskNotExpired {});
    }

    task.status = TaskStatus::Expired;
    TASKS.save(deps.storage, task_id, &task)?;

    // Refund escrow to poster
    let refund_msg = BankMsg::Send {
        to_address: task.poster.to_string(),
        amount: vec![task.escrow.clone()],
    };

    Ok(Response::new()
        .add_message(refund_msg)
        .add_attribute("method", "expire_task")
        .add_attribute("task_id", task_id.to_string())
        .add_attribute("escrow_refunded", format!("{}{}", task.escrow.amount, task.escrow.denom)))
}

fn execute_update_config(
    deps: DepsMut,
    info: MessageInfo,
    minimum_escrow: Option<Uint128>,
) -> Result<Response, ContractError> {
    let mut config = TASK_CONFIG.load(deps.storage)?;
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized {});
    }

    if let Some(min) = minimum_escrow {
        config.minimum_escrow = min;
    }

    TASK_CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("method", "update_config")
        .add_attribute("minimum_escrow", config.minimum_escrow))
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
        category: task.category.clone(),
        specializations: task.specializations.clone(),
        escrow: task.escrow.clone(),
        status: task.status.clone(),
        claimant: task.claimant.clone(),
        created_at: task.created_at,
        claimed_at: task.claimed_at,
        submitted_at: task.submitted_at,
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
        minimum_escrow: config.minimum_escrow,
        denom: config.denom,
    })
}
