use cosmwasm_schema::{QueryResponses, cw_serde};
use tidepool_types::SkillInput;

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
    #[returns(tidepool_types::AgentResponse)]
    GetAgent { address: String },
    #[returns(tidepool_types::AgentsListResponse)]
    ListAgents {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    #[returns(tidepool_types::LeaderboardResponse)]
    Leaderboard {
        limit: Option<u32>,
        skill: Option<String>,
    },
    #[returns(tidepool_types::AgentsListResponse)]
    GetAgentsBySkill {
        skill: String,
        min_rating: Option<u8>,
        limit: Option<u32>,
    },
    #[returns(tidepool_types::ReputationConfigResponse)]
    Config {},
}
