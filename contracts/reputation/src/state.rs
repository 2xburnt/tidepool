use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use tidepool_types::Badge;

#[cw_serde]
pub struct Config {
    pub owner: Addr,
    pub task_contract: Option<Addr>,
}

#[cw_serde]
pub struct Agent {
    pub name: String,
    pub level: u64,
    pub xp: u64,
    pub badges: Vec<Badge>,
    pub tasks_completed: u64,
    pub tasks_posted: u64,
    pub registered_at: u64,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const AGENTS: Map<&Addr, Agent> = Map::new("agents");
pub const ISSUERS: Map<&Addr, bool> = Map::new("issuers");
pub const AGENT_COUNT: Item<u64> = Item::new("agent_count");
