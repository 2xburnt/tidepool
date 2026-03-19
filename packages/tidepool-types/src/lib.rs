use cosmwasm_schema::{QueryResponses, cw_serde};
use cosmwasm_std::{Addr, Coin, Uint128};

// ============ Skill Types ============

/// Input for agent registration — just name + self-rating
#[cw_serde]
pub struct SkillInput {
    pub name: String,
    /// Self-declared rating 1-5
    pub rating: u8,
}

/// Full skill record stored on-chain
#[cw_serde]
pub struct Skill {
    pub name: String,
    /// Self-declared rating 1-5
    pub self_rating: u8,
    /// Jobs completed with this skill tag
    pub jobs_completed: u64,
    /// XION earned on jobs with this skill tag
    pub total_earned: Uint128,
}

// ============ Reputation / Agent Types ============

#[cw_serde]
pub struct AgentResponse {
    pub address: Addr,
    pub name: String,
    pub skills: Vec<Skill>,
    pub total_earned: Uint128,
    pub total_spent: Uint128,
    pub jobs_completed: u64,
    pub jobs_posted: u64,
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
    UpdateVolume {
        worker: String,
        poster: String,
        amount: Uint128,
        /// The task's required skills — used to increment per-skill stats on the worker
        skills: Vec<String>,
    },
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
    Submitted,
    Completed,
    Cancelled,
    Expired,
}

#[cw_serde]
pub struct TaskResponse {
    pub id: u64,
    pub poster: Addr,
    pub title: String,
    pub description: String,
    pub required_skills: Vec<String>,
    pub escrow: Coin,
    pub status: TaskStatus,
    pub claimant: Option<Addr>,
    pub created_at: u64,
    pub claimed_at: Option<u64>,
    pub submitted_at: Option<u64>,
    pub completed_at: Option<u64>,
    pub expires_at: Option<u64>,
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
    pub minimum_escrow: Uint128,
    pub denom: String,
}

/// Default minimum escrow: 0.1 XION = 100_000 uxion
pub const DEFAULT_MINIMUM_ESCROW: u128 = 100_000;
pub const ESCROW_DENOM: &str = "uxion";

/// Auto-release window: 24 hours in seconds
pub const AUTO_RELEASE_SECONDS: u64 = 86_400;
