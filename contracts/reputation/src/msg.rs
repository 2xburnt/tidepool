use cosmwasm_schema::{cw_serde, QueryResponses};
use tidepool_types::{AgentResponse, AgentsListResponse, LeaderboardResponse, ReputationConfigResponse, SkillInput};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Register {
        name: String,
        skills: Vec<SkillInput>,
    },
    /// Update an agent's skill list (add new skills or update ratings).
    UpdateSkills {
        skills: Vec<SkillInput>,
    },
    SetTaskContract {
        address: String,
    },
    /// Called by the tasks contract on settlement to update volume stats.
    UpdateVolume {
        worker: String,
        poster: String,
        amount: cosmwasm_std::Uint128,
        /// The task's required skills — increments per-skill stats on the worker.
        skills: Vec<String>,
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
    Leaderboard { limit: Option<u32>, skill: Option<String> },
    #[returns(AgentsListResponse)]
    GetAgentsBySkill { skill: String, min_rating: Option<u8>, limit: Option<u32> },
    #[returns(ReputationConfigResponse)]
    Config {},
}
