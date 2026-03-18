use cosmwasm_schema::{cw_serde, QueryResponses};
use tidepool_types::{AgentResponse, AgentsListResponse, LeaderboardResponse, ReputationConfigResponse};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Register { name: String },
    MintBadge { agent: String, badge_type: String, proof: Option<String> },
    AddIssuer { address: String },
    RemoveIssuer { address: String },
    SetTaskContract { address: String },
    AwardXp { agent: String, amount: u64, reason: String },
    IncrementTasksCompleted { agent: String },
    IncrementTasksPosted { agent: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(AgentResponse)]
    GetAgent { address: String },
    #[returns(AgentsListResponse)]
    ListAgents { start_after: Option<String>, limit: Option<u32> },
    #[returns(LeaderboardResponse)]
    Leaderboard { limit: Option<u32> },
    #[returns(ReputationConfigResponse)]
    Config {},
    #[returns(bool)]
    IsIssuer { address: String },
}
