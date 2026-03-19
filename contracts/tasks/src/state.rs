use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, Uint128};
use cw_storage_plus::{Item, Map};
use tidepool_types::TaskStatus;

#[cw_serde]
pub struct TaskConfig {
    pub owner: Addr,
    pub reputation_contract: Addr,
    pub next_task_id: u64,
    pub minimum_escrow: Uint128,
    pub denom: String,
}

#[cw_serde]
pub struct Task {
    pub id: u64,
    pub poster: Addr,
    pub title: String,
    pub description: String,
    pub required_skills: Vec<String>,
    pub escrow: Coin,
    pub status: TaskStatus,
    pub claimant: Option<Addr>,
    pub created_at: u64,           // block height
    pub claimed_at: Option<u64>,   // block height
    pub submitted_at: Option<u64>, // block time (seconds) for auto-release
    pub completed_at: Option<u64>, // block height
    pub expires_at: Option<u64>,   // block height
}

pub const TASK_CONFIG: Item<TaskConfig> = Item::new("task_config");
pub const TASKS: Map<u64, Task> = Map::new("tasks");
