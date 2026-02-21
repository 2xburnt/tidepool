use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Agent, AGENTS, CONFIG, Config, ISSUERS};

const CONTRACT_NAME: &str = "crates.io:tidepool-reputation";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let config = Config {
        owner: info.sender.clone(),
    };
    CONFIG.save(deps.storage, &config)?;

    ISSUERS.save(deps.storage, &info.sender, &true)?; // Owner is an issuer by default

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Register { name } => register(deps, info, name),
        ExecuteMsg::MintBadge { agent, badge_type, proof } => mint_badge(deps, info, agent, badge_type, proof),
        ExecuteMsg::AddIssuer { address } => add_issuer(deps, info, address),
        ExecuteMsg::RemoveIssuer { address } => remove_issuer(deps, info, address),
    }
}

pub fn register(deps: DepsMut, info: MessageInfo, name: String) -> StdResult<Response> {
    // Check if already registered
    if AGENTS.may_load(deps.storage, &info.sender)?.is_some() {
        return Err(cosmwasm_std::StdError::generic_err("Already registered"));
    }

    let agent = Agent {
        name: name.clone(),
        level: 1,
        xp: 0,
        badges: vec![],
    };

    AGENTS.save(deps.storage, &info.sender, &agent)?;

    Ok(Response::new()
        .add_attribute("method", "register")
        .add_attribute("agent", info.sender)
        .add_attribute("name", name))
}

pub fn mint_badge(
    deps: DepsMut, 
    info: MessageInfo, 
    agent_addr: String, 
    badge_type: String, 
    _proof: String
) -> StdResult<Response> {
    // Check if sender is an authorized issuer
    if !ISSUERS.may_load(deps.storage, &info.sender)?.unwrap_or(false) {
        return Err(cosmwasm_std::StdError::generic_err("Unauthorized: Not an issuer"));
    }
    
    let agent_addr_validated = deps.api.addr_validate(&agent_addr)?;
    
    AGENTS.update(deps.storage, &agent_addr_validated, |agent| -> StdResult<_> {
        let mut agent = agent.ok_or_else(|| cosmwasm_std::StdError::generic_err("Agent not found"))?;
        if !agent.badges.contains(&badge_type) {
             agent.badges.push(badge_type.clone());
        }
        Ok(agent)
    })?;

    Ok(Response::new()
        .add_attribute("method", "mint_badge")
        .add_attribute("agent", agent_addr)
        .add_attribute("badge", badge_type)
        .add_attribute("issuer", info.sender))
}

pub fn add_issuer(deps: DepsMut, info: MessageInfo, address: String) -> StdResult<Response> {
    let config = CONFIG.load(deps.storage)?;
    if info.sender != config.owner {
        return Err(cosmwasm_std::StdError::generic_err("Unauthorized: Only owner can add issuers"));
    }

    let addr = deps.api.addr_validate(&address)?;
    ISSUERS.save(deps.storage, &addr, &true)?;

    Ok(Response::new()
        .add_attribute("method", "add_issuer")
        .add_attribute("issuer", address))
}

pub fn remove_issuer(deps: DepsMut, info: MessageInfo, address: String) -> StdResult<Response> {
    let config = CONFIG.load(deps.storage)?;
    if info.sender != config.owner {
        return Err(cosmwasm_std::StdError::generic_err("Unauthorized: Only owner can remove issuers"));
    }

    let addr = deps.api.addr_validate(&address)?;
    ISSUERS.remove(deps.storage, &addr);

    Ok(Response::new()
        .add_attribute("method", "remove_issuer")
        .add_attribute("issuer", address))
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetAgent { address } => to_json_binary(&query_agent(deps, address)?),
    }
}

fn query_agent(deps: Deps, address: String) -> StdResult<super::msg::AgentResponse> {
    let addr = deps.api.addr_validate(&address)?;
    let agent = AGENTS.load(deps.storage, &addr)?;
    Ok(super::msg::AgentResponse {
        address: addr,
        name: agent.name,
        level: agent.level,
        xp: agent.xp,
        badges: agent.badges,
    })
}
