use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Order, Response,
    StdResult, Uint128,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Agent, Config, AGENTS, AGENT_COUNT, CONFIG};
use tidepool_types::{
    AgentResponse, AgentsListResponse, LeaderboardResponse, ReputationConfigResponse,
    Skill, SkillInput,
};

const CONTRACT_NAME: &str = "crates.io:tidepool-reputation";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// ─── Helpers ───

fn validate_skill_inputs(inputs: &[SkillInput]) -> Result<(), ContractError> {
    for s in inputs {
        if s.rating < 1 || s.rating > 5 {
            return Err(ContractError::InvalidSkillRating { rating: s.rating });
        }
    }
    Ok(())
}

fn skills_from_inputs(inputs: &[SkillInput]) -> Vec<Skill> {
    inputs
        .iter()
        .map(|si| Skill {
            name: si.name.clone(),
            self_rating: si.rating,
            jobs_completed: 0,
            total_earned: Uint128::zero(),
        })
        .collect()
}

fn agent_to_response(addr: &cosmwasm_std::Addr, agent: &Agent) -> AgentResponse {
    AgentResponse {
        address: addr.clone(),
        name: agent.name.clone(),
        skills: agent.skills.clone(),
        total_earned: agent.total_earned,
        total_spent: agent.total_spent,
        jobs_completed: agent.jobs_completed,
        jobs_posted: agent.jobs_posted,
        registered_at: agent.registered_at,
    }
}

// ─── Entry Points ───

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
        ExecuteMsg::Register { name, skills } => execute_register(deps, env, info, name, skills),
        ExecuteMsg::UpdateSkills { skills } => execute_update_skills(deps, info, skills),
        ExecuteMsg::SetTaskContract { address } => execute_set_task_contract(deps, info, address),
        ExecuteMsg::UpdateVolume {
            worker,
            poster,
            amount,
            skills,
        } => execute_update_volume(deps, info, worker, poster, amount, skills),
    }
}

fn execute_register(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    name: String,
    skill_inputs: Vec<SkillInput>,
) -> Result<Response, ContractError> {
    if AGENTS.may_load(deps.storage, &info.sender)?.is_some() {
        return Err(ContractError::AlreadyRegistered {});
    }

    validate_skill_inputs(&skill_inputs)?;

    let agent = Agent {
        name: name.clone(),
        skills: skills_from_inputs(&skill_inputs),
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

fn execute_update_skills(
    deps: DepsMut,
    info: MessageInfo,
    skill_inputs: Vec<SkillInput>,
) -> Result<Response, ContractError> {
    validate_skill_inputs(&skill_inputs)?;

    AGENTS.update(deps.storage, &info.sender, |agent| -> Result<_, ContractError> {
        let mut agent = agent.ok_or(ContractError::AgentNotFound {})?;

        for input in &skill_inputs {
            if let Some(existing) = agent.skills.iter_mut().find(|s| s.name == input.name) {
                // Update rating, keep stats
                existing.self_rating = input.rating;
            } else {
                // Add new skill
                agent.skills.push(Skill {
                    name: input.name.clone(),
                    self_rating: input.rating,
                    jobs_completed: 0,
                    total_earned: Uint128::zero(),
                });
            }
        }
        Ok(agent)
    })?;

    Ok(Response::new()
        .add_attribute("method", "update_skills")
        .add_attribute("agent", info.sender))
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
    skills: Vec<String>,
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

    // Update worker stats + per-skill stats
    AGENTS.update(deps.storage, &worker, |agent| -> Result<_, ContractError> {
        let mut agent = agent.ok_or(ContractError::AgentNotFound {})?;
        agent.total_earned += amount;
        agent.jobs_completed += 1;

        // Increment per-skill stats for matching skills
        for skill_name in &skills {
            if let Some(skill) = agent.skills.iter_mut().find(|s| &s.name == skill_name) {
                skill.jobs_completed += 1;
                skill.total_earned += amount;
            }
        }

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

// ─── Queries ───

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetAgent { address } => to_json_binary(&query_agent(deps, address)?),
        QueryMsg::ListAgents { start_after, limit } => {
            to_json_binary(&query_list_agents(deps, start_after, limit)?)
        }
        QueryMsg::Leaderboard { limit, skill } => {
            to_json_binary(&query_leaderboard(deps, limit, skill)?)
        }
        QueryMsg::GetAgentsBySkill {
            skill,
            min_rating,
            limit,
        } => to_json_binary(&query_agents_by_skill(deps, skill, min_rating, limit)?),
        QueryMsg::Config {} => to_json_binary(&query_config(deps)?),
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

fn query_leaderboard(
    deps: Deps,
    limit: Option<u32>,
    skill: Option<String>,
) -> StdResult<LeaderboardResponse> {
    let limit = limit.unwrap_or(10).min(100) as usize;

    let mut agents: Vec<AgentResponse> = AGENTS
        .range(deps.storage, None, None, Order::Ascending)
        .map(|item| {
            let (addr, agent) = item?;
            Ok(agent_to_response(&addr, &agent))
        })
        .collect::<StdResult<_>>()?;

    match skill {
        Some(ref skill_name) => {
            // Filter to agents who have this skill, sort by total_earned within that skill
            agents.retain(|a| a.skills.iter().any(|s| &s.name == skill_name));
            agents.sort_by(|a, b| {
                let a_earned = a
                    .skills
                    .iter()
                    .find(|s| &s.name == skill_name)
                    .map(|s| s.total_earned)
                    .unwrap_or_default();
                let b_earned = b
                    .skills
                    .iter()
                    .find(|s| &s.name == skill_name)
                    .map(|s| s.total_earned)
                    .unwrap_or_default();
                b_earned.cmp(&a_earned)
            });
        }
        None => {
            // Sort by overall total_earned descending
            agents.sort_by(|a, b| b.total_earned.cmp(&a.total_earned));
        }
    }

    agents.truncate(limit);
    Ok(LeaderboardResponse { agents })
}

fn query_agents_by_skill(
    deps: Deps,
    skill: String,
    min_rating: Option<u8>,
    limit: Option<u32>,
) -> StdResult<AgentsListResponse> {
    let limit = limit.unwrap_or(30).min(100) as usize;
    let min_rating = min_rating.unwrap_or(1);

    let agents: Vec<AgentResponse> = AGENTS
        .range(deps.storage, None, None, Order::Ascending)
        .filter_map(|item| {
            let (addr, agent) = item.ok()?;
            let has_skill = agent
                .skills
                .iter()
                .any(|s| s.name == skill && s.self_rating >= min_rating);
            if has_skill {
                Some(agent_to_response(&addr, &agent))
            } else {
                None
            }
        })
        .take(limit)
        .collect();

    Ok(AgentsListResponse { agents })
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
