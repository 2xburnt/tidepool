use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin};

// ============ Reputation Types ============

#[cw_serde]
pub struct Badge {
    pub badge_type: String,
    pub issuer: Addr,
    pub issued_at: u64,
    pub proof: Option<String>,
}

#[cw_serde]
pub struct AgentResponse {
    pub address: Addr,
    pub name: String,
    pub level: u64,
    pub xp: u64,
    pub badges: Vec<Badge>,
    pub tasks_completed: u64,
    pub tasks_posted: u64,
    pub registered_at: u64,
}

#[cw_serde]
pub struct AgentsListResponse {
    pub agents: Vec<AgentResponse>,
}

#[cw_serde]
pub struct LeaderboardResponse {
    pub agents: Vec<AgentResponse>,
}

#[cw_serde]
pub struct ReputationConfigResponse {
    pub owner: Addr,
    pub task_contract: Option<Addr>,
    pub agent_count: u64,
}

// ============ Reputation Messages (for cross-contract calls) ============

/// Execute messages that the Task contract sends to the Reputation contract
#[cw_serde]
pub enum ReputationExecuteMsg {
    AwardXp { agent: String, amount: u64, reason: String },
    IncrementTasksCompleted { agent: String },
    IncrementTasksPosted { agent: String },
}

/// Query messages for cross-contract queries to Reputation contract
#[cw_serde]
#[derive(QueryResponses)]
pub enum ReputationQueryMsg {
    #[returns(AgentResponse)]
    GetAgent { address: String },
}

// ============ Task Types ============

#[cw_serde]
pub enum TaskStatus {
    Open,
    Claimed,
    Completed,
    Expired,
}

#[cw_serde]
pub struct TaskResponse {
    pub id: u64,
    pub poster: Addr,
    pub title: String,
    pub description: String,
    pub xp_reward: u64,
    pub required_badges: Vec<String>,
    pub status: TaskStatus,
    pub claimant: Option<Addr>,
    pub created_at: u64,
    pub claimed_at: Option<u64>,
    pub completed_at: Option<u64>,
    pub expires_at: Option<u64>,
    pub bounty: Option<Coin>,
}

#[cw_serde]
pub struct TasksListResponse {
    pub tasks: Vec<TaskResponse>,
}

#[cw_serde]
pub struct TaskConfigResponse {
    pub owner: Addr,
    pub reputation_contract: Addr,
    pub next_task_id: u64,
}

// ============ XP Constants ============

pub const XP_REGISTER: u64 = 10;
pub const XP_TASK_COMPLETE: u64 = 50;
pub const XP_TASK_POSTED_COMPLETED: u64 = 20;
pub const XP_BADGE_EARNED: u64 = 25;

/// Returns the level for a given XP amount
pub fn level_for_xp(xp: u64) -> u64 {
    match xp {
        0..100 => 1,
        100..250 => 2,
        250..500 => 3,
        500..1000 => 4,
        1000..2000 => 5,
        2000..3500 => 6,
        3500..5500 => 7,
        5500..8000 => 8,
        8000..12000 => 9,
        _ => 10,
    }
}
