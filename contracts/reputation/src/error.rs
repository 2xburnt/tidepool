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
}
