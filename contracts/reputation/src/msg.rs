use cosmwasm_schema::{cw_serde, QueryResponses};
use tidepool_types::{AgentResponse, AgentsListResponse, LeaderboardResponse, ReputationConfigResponse};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Register {
        name: String,
        specializations: Vec<String>,
    },
    SetTaskContract {
        address: String,
    },
    /// Called by the tasks contract on settlement to update volume stats
    UpdateVolume {
        worker: String,
        poster: String,
        amount: cosmwasm_std::Uint128,
    },
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
}
