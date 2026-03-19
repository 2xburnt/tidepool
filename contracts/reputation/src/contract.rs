use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Order, Response,
    StdResult, Uint128,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Agent, Config, AGENTS, AGENT_COUNT, CONFIG};
use tidepool_types::{AgentResponse, AgentsListResponse, LeaderboardResponse, ReputationConfigResponse};

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
        ExecuteMsg::Register {
            name,
            specializations,
        } => execute_register(deps, env, info, name, specializations),
        ExecuteMsg::SetTaskContract { address } => execute_set_task_contract(deps, info, address),
        ExecuteMsg::UpdateVolume {
            worker,
            poster,
            amount,
        } => execute_update_volume(deps, info, worker, poster, amount),
    }
}

fn execute_register(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    name: String,
    specializations: Vec<String>,
) -> Result<Response, ContractError> {
    if AGENTS.may_load(deps.storage, &info.sender)?.is_some() {
        return Err(ContractError::AlreadyRegistered {});
    }

    let agent = Agent {
        name: name.clone(),
        specializations: specializations.clone(),
        total_earned: Uint128::zero(),
        total_spent: Uint128::zero(),
        jobs_completed: 0,
        jobs_posted: 0,
        registered_at: env.block.height,
    };

    AGENTS.save(deps.storage, &info.sender, &agent)?;
    AGENT_COUNT.update(deps.storage, |c| -> StdResult<_> { Ok(c + 1) })?;

    Ok(Response::new()
        .add_attribute("method", "register")
        .add_attribute("agent", info.sender)
        .add_attribute("name", name))
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

fn execute_update_volume(
    deps: DepsMut,
    info: MessageInfo,
    worker_addr: String,
    poster_addr: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    // Only the task contract or owner can call this
    let config = CONFIG.load(deps.storage)?;
    let authorized = info.sender == config.owner
        || config.task_contract.as_ref() == Some(&info.sender);
    if !authorized {
        return Err(ContractError::Unauthorized {});
    }

    let worker = deps.api.addr_validate(&worker_addr)?;
    let poster = deps.api.addr_validate(&poster_addr)?;

    // Update worker stats
    AGENTS.update(deps.storage, &worker, |agent| -> Result<_, ContractError> {
        let mut agent = agent.ok_or(ContractError::AgentNotFound {})?;
        agent.total_earned += amount;
        agent.jobs_completed += 1;
        Ok(agent)
    })?;

    // Update poster stats
    AGENTS.update(deps.storage, &poster, |agent| -> Result<_, ContractError> {
        let mut agent = agent.ok_or(ContractError::AgentNotFound {})?;
        agent.total_spent += amount;
        agent.jobs_posted += 1;
        Ok(agent)
    })?;

    Ok(Response::new()
        .add_attribute("method", "update_volume")
        .add_attribute("worker", worker_addr)
        .add_attribute("poster", poster_addr)
        .add_attribute("amount", amount))
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
    }
}

fn agent_to_response(addr: &cosmwasm_std::Addr, agent: &Agent) -> AgentResponse {
    AgentResponse {
        address: addr.clone(),
        name: agent.name.clone(),
        specializations: agent.specializations.clone(),
        total_earned: agent.total_earned,
        total_spent: agent.total_spent,
        jobs_completed: agent.jobs_completed,
        jobs_posted: agent.jobs_posted,
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

    // Sort by total_earned descending (reputation = volume)
    agents.sort_by(|a, b| b.total_earned.cmp(&a.total_earned));
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
