use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub owner: Addr, // Only owner can add verified issuers initially
}

#[cw_serde]
pub struct Agent {
    pub name: String,
    pub level: u64,
    pub xp: u64,
    pub badges: Vec<String>,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const AGENTS: Map<&Addr, Agent> = Map::new("agents");
pub const ISSUERS: Map<&Addr, bool> = Map::new("issuers");
