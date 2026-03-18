use cosmwasm_schema::{cw_serde, QueryResponses};
use tidepool_types::{TaskConfigResponse, TaskResponse, TaskStatus, TasksListResponse};

#[cw_serde]
pub struct InstantiateMsg {
    pub reputation_contract: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    PostTask {
        title: String,
        description: String,
        xp_reward: u64,
        required_badges: Vec<String>,
        expires_in_blocks: Option<u64>,
    },
    ClaimTask { task_id: u64 },
    CompleteTask { task_id: u64 },
    ExpireTask { task_id: u64 },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(TaskResponse)]
    GetTask { task_id: u64 },
    #[returns(TasksListResponse)]
    ListTasks { status: Option<TaskStatus>, start_after: Option<u64>, limit: Option<u32> },
    #[returns(TasksListResponse)]
    MyPostedTasks { address: String },
    #[returns(TasksListResponse)]
    MyClaimedTasks { address: String },
    #[returns(TaskConfigResponse)]
    Config {},
}
