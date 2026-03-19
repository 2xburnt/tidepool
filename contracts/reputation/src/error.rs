use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Agent already registered")]
    AlreadyRegistered {},

    #[error("Agent not found")]
    AgentNotFound {},

    #[error("Not an authorized issuer")]
    NotIssuer {},

    #[error("Invalid skill rating: must be 1-5, got {rating}")]
    InvalidSkillRating { rating: u8 },
}
