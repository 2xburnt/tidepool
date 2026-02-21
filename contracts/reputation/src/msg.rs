use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Register { name: String },
    // Only verified issuers (or zkTLS proof verification) can mint badges
    MintBadge { 
        agent: String, 
        badge_type: String, 
        proof: String 
    },
    AddIssuer { address: String },
    RemoveIssuer { address: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(AgentResponse)]
    GetAgent { address: String },
}

#[cw_serde]
pub struct AgentResponse {
    pub address: Addr,
    pub name: String,
    pub level: u64,
    pub xp: u64,
    pub badges: Vec<String>,
}
