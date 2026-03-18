use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Order, Response, StdResult,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Agent, Config, AGENTS, AGENT_COUNT, CONFIG, ISSUERS};
use tidepool_types::{
    level_for_xp, AgentResponse, AgentsListResponse, Badge, LeaderboardResponse,
    ReputationConfigResponse, XP_BADGE_EARNED, XP_REGISTER,
};

const CONTRACT_NAME: &str = "crates.io:tidepool-reputation";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let config = Config {
        owner: info.sender.clone(),
        task_contract: None,
    };
    CONFIG.save(deps.storage, &config)?;
    AGENT_COUNT.save(deps.storage, &0u64)?;
    ISSUERS.save(deps.storage, &info.sender, &true)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Register { name } => execute_register(deps, env, info, name),
        ExecuteMsg::MintBadge { agent, badge_type, proof } => {
            execute_mint_badge(deps, env, info, agent, badge_type, proof)
        }
        ExecuteMsg::AddIssuer { address } => execute_add_issuer(deps, info, address),
        ExecuteMsg::RemoveIssuer { address } => execute_remove_issuer(deps, info, address),
        ExecuteMsg::SetTaskContract { address } => execute_set_task_contract(deps, info, address),
        ExecuteMsg::AwardXp { agent, amount, reason } => {
            execute_award_xp(deps, info, agent, amount, reason)
        }
        ExecuteMsg::IncrementTasksCompleted { agent } => {
            execute_increment_tasks_completed(deps, info, agent)
        }
        ExecuteMsg::IncrementTasksPosted { agent } => {
            execute_increment_tasks_posted(deps, info, agent)
        }
    }
}

fn execute_register(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    name: String,
) -> Result<Response, ContractError> {
    if AGENTS.may_load(deps.storage, &info.sender)?.is_some() {
        return Err(ContractError::AlreadyRegistered {});
    }

    let agent = Agent {
        name: name.clone(),
        level: 1,
        xp: XP_REGISTER,
        badges: vec![],
        tasks_completed: 0,
        tasks_posted: 0,
        registered_at: env.block.height,
    };

    AGENTS.save(deps.storage, &info.sender, &agent)?;
    AGENT_COUNT.update(deps.storage, |c| -> StdResult<_> { Ok(c + 1) })?;

    Ok(Response::new()
        .add_attribute("method", "register")
        .add_attribute("agent", info.sender)
        .add_attribute("name", name)
        .add_attribute("xp_awarded", XP_REGISTER.to_string()))
}

fn execute_mint_badge(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    agent_addr: String,
    badge_type: String,
    proof: Option<String>,
) -> Result<Response, ContractError> {
    if !ISSUERS.may_load(deps.storage, &info.sender)?.unwrap_or(false) {
        return Err(ContractError::NotIssuer {});
    }

    let addr = deps.api.addr_validate(&agent_addr)?;

    AGENTS.update(deps.storage, &addr, |agent| -> Result<_, ContractError> {
        let mut agent = agent.ok_or(ContractError::AgentNotFound {})?;

        if !agent.badges.iter().any(|b| b.badge_type == badge_type) {
            agent.badges.push(Badge {
                badge_type: badge_type.clone(),
                issuer: info.sender.clone(),
                issued_at: env.block.height,
                proof,
            });
            agent.xp += XP_BADGE_EARNED;
            agent.level = level_for_xp(agent.xp);
        }
        Ok(agent)
    })?;

    Ok(Response::new()
        .add_attribute("method", "mint_badge")
        .add_attribute("agent", agent_addr)
        .add_attribute("badge", badge_type)
        .add_attribute("issuer", info.sender))
}

fn execute_add_issuer(
    deps: DepsMut,
    info: MessageInfo,
    address: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized {});
    }
    let addr = deps.api.addr_validate(&address)?;
    ISSUERS.save(deps.storage, &addr, &true)?;
    Ok(Response::new()
        .add_attribute("method", "add_issuer")
        .add_attribute("issuer", address))
}

fn execute_remove_issuer(
    deps: DepsMut,
    info: MessageInfo,
    address: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized {});
    }
    let addr = deps.api.addr_validate(&address)?;
    ISSUERS.remove(deps.storage, &addr);
    Ok(Response::new()
        .add_attribute("method", "remove_issuer")
        .add_attribute("issuer", address))
}

fn execute_set_task_contract(
    deps: DepsMut,
    info: MessageInfo,
    address: String,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized {});
    }
    let addr = deps.api.addr_validate(&address)?;
    config.task_contract = Some(addr.clone());
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new()
        .add_attribute("method", "set_task_contract")
        .add_attribute("task_contract", addr))
}

fn is_authorized_caller(deps: &DepsMut, info: &MessageInfo) -> Result<bool, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if info.sender == config.owner {
        return Ok(true);
    }
    if let Some(tc) = config.task_contract {
        if info.sender == tc {
            return Ok(true);
        }
    }
    Ok(false)
}

fn execute_award_xp(
    deps: DepsMut,
    info: MessageInfo,
    agent_addr: String,
    amount: u64,
    reason: String,
) -> Result<Response, ContractError> {
    if !is_authorized_caller(&deps, &info)? {
        return Err(ContractError::Unauthorized {});
    }
    let addr = deps.api.addr_validate(&agent_addr)?;
    AGENTS.update(deps.storage, &addr, |agent| -> Result<_, ContractError> {
        let mut agent = agent.ok_or(ContractError::AgentNotFound {})?;
        agent.xp += amount;
        agent.level = level_for_xp(agent.xp);
        Ok(agent)
    })?;
    Ok(Response::new()
        .add_attribute("method", "award_xp")
        .add_attribute("agent", agent_addr)
        .add_attribute("amount", amount.to_string())
        .add_attribute("reason", reason))
}

fn execute_increment_tasks_completed(
    deps: DepsMut,
    info: MessageInfo,
    agent_addr: String,
) -> Result<Response, ContractError> {
    if !is_authorized_caller(&deps, &info)? {
        return Err(ContractError::Unauthorized {});
    }
    let addr = deps.api.addr_validate(&agent_addr)?;
    AGENTS.update(deps.storage, &addr, |agent| -> Result<_, ContractError> {
        let mut agent = agent.ok_or(ContractError::AgentNotFound {})?;
        agent.tasks_completed += 1;
        Ok(agent)
    })?;
    Ok(Response::new()
        .add_attribute("method", "increment_tasks_completed")
        .add_attribute("agent", agent_addr))
}

fn execute_increment_tasks_posted(
    deps: DepsMut,
    info: MessageInfo,
    agent_addr: String,
) -> Result<Response, ContractError> {
    if !is_authorized_caller(&deps, &info)? {
        return Err(ContractError::Unauthorized {});
    }
    let addr = deps.api.addr_validate(&agent_addr)?;
    AGENTS.update(deps.storage, &addr, |agent| -> Result<_, ContractError> {
        let mut agent = agent.ok_or(ContractError::AgentNotFound {})?;
        agent.tasks_posted += 1;
        Ok(agent)
    })?;
    Ok(Response::new()
        .add_attribute("method", "increment_tasks_posted")
        .add_attribute("agent", agent_addr))
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetAgent { address } => to_json_binary(&query_agent(deps, address)?),
        QueryMsg::ListAgents { start_after, limit } => {
            to_json_binary(&query_list_agents(deps, start_after, limit)?)
        }
        QueryMsg::Leaderboard { limit } => to_json_binary(&query_leaderboard(deps, limit)?),
        QueryMsg::Config {} => to_json_binary(&query_config(deps)?),
        QueryMsg::IsIssuer { address } => to_json_binary(&query_is_issuer(deps, address)?),
    }
}

fn agent_to_response(addr: &cosmwasm_std::Addr, agent: &Agent) -> AgentResponse {
    AgentResponse {
        address: addr.clone(),
        name: agent.name.clone(),
        level: agent.level,
        xp: agent.xp,
        badges: agent.badges.clone(),
        tasks_completed: agent.tasks_completed,
        tasks_posted: agent.tasks_posted,
        registered_at: agent.registered_at,
    }
}

fn query_agent(deps: Deps, address: String) -> StdResult<AgentResponse> {
    let addr = deps.api.addr_validate(&address)?;
    let agent = AGENTS.load(deps.storage, &addr)?;
    Ok(agent_to_response(&addr, &agent))
}

fn query_list_agents(
    deps: Deps,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<AgentsListResponse> {
    let limit = limit.unwrap_or(30).min(100) as usize;
    let start = start_after
        .as_ref()
        .map(|s| deps.api.addr_validate(s))
        .transpose()?;

    let agents: Vec<AgentResponse> = AGENTS
        .range(
            deps.storage,
            start.as_ref().map(cw_storage_plus::Bound::exclusive),
            None,
            Order::Ascending,
        )
        .take(limit)
        .map(|item| {
            let (addr, agent) = item?;
            Ok(agent_to_response(&addr, &agent))
        })
        .collect::<StdResult<_>>()?;

    Ok(AgentsListResponse { agents })
}

fn query_leaderboard(deps: Deps, limit: Option<u32>) -> StdResult<LeaderboardResponse> {
    let limit = limit.unwrap_or(10).min(100) as usize;

    let mut agents: Vec<AgentResponse> = AGENTS
        .range(deps.storage, None, None, Order::Ascending)
        .map(|item| {
            let (addr, agent) = item?;
            Ok(agent_to_response(&addr, &agent))
        })
        .collect::<StdResult<_>>()?;

    agents.sort_by(|a, b| b.xp.cmp(&a.xp));
    agents.truncate(limit);

    Ok(LeaderboardResponse { agents })
}

fn query_config(deps: Deps) -> StdResult<ReputationConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    let count = AGENT_COUNT.load(deps.storage)?;
    Ok(ReputationConfigResponse {
        owner: config.owner,
        task_contract: config.task_contract,
        agent_count: count,
    })
}

fn query_is_issuer(deps: Deps, address: String) -> StdResult<bool> {
    let addr = deps.api.addr_validate(&address)?;
    Ok(ISSUERS.may_load(deps.storage, &addr)?.unwrap_or(false))
}
