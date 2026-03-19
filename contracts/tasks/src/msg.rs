use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;
use tidepool_types::{TaskConfigResponse, TaskResponse, TaskStatus, TasksListResponse};

#[cw_serde]
pub struct InstantiateMsg {
    pub reputation_contract: String,
    /// Minimum escrow in uxion. Defaults to 100_000 (0.1 XION).
    pub minimum_escrow: Option<Uint128>,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Post a new task. Must send uxion funds >= minimum_escrow.
    /// Poster must be registered in the reputation contract.
    PostTask {
        title: String,
        description: String,
        required_skills: Vec<String>,
        expires_in_blocks: Option<u64>,
    },
    /// Worker claims an open task.
    ClaimTask { task_id: u64 },
    /// Worker submits completed work.
    SubmitTask { task_id: u64 },
    /// Poster approves submission → settle escrow to worker.
    ApproveTask { task_id: u64 },
    /// Anyone can call after 24h past submission → auto-release escrow to worker.
    AutoRelease { task_id: u64 },
    /// Poster cancels (only if unclaimed) → refund escrow.
    CancelTask { task_id: u64 },
    /// Anyone can expire an unclaimed task past its expiry block.
    ExpireTask { task_id: u64 },
    /// Admin updates config (minimum escrow).
    UpdateConfig { minimum_escrow: Option<Uint128> },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(TaskResponse)]
    GetTask { task_id: u64 },
    #[returns(TasksListResponse)]
    ListTasks {
        status: Option<TaskStatus>,
        start_after: Option<u64>,
        limit: Option<u32>,
    },
    #[returns(TasksListResponse)]
    MyPostedTasks { address: String },
    #[returns(TasksListResponse)]
    MyClaimedTasks { address: String },
    #[returns(TaskConfigResponse)]
    Config {},
}
