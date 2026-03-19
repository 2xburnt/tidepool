use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};
use tidepool_types::Skill;

#[cw_serde]
pub struct Config {
    pub owner: Addr,
    pub task_contract: Option<Addr>,
}

#[cw_serde]
pub struct Agent {
    pub name: String,
    pub skills: Vec<Skill>,
    pub total_earned: Uint128,
    pub total_spent: Uint128,
    pub jobs_completed: u64,
    pub jobs_posted: u64,
    pub registered_at: u64,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const AGENTS: Map<&Addr, Agent> = Map::new("agents");
pub const AGENT_COUNT: Item<u64> = Item::new("agent_count");
