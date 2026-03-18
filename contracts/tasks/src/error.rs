use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Task not found")]
    TaskNotFound {},

    #[error("Task is not open")]
    TaskNotOpen {},

    #[error("Task is not claimed")]
    TaskNotClaimed {},

    #[error("Task has not expired")]
    TaskNotExpired {},

    #[error("Agent not registered in reputation contract")]
    AgentNotRegistered {},

    #[error("Agent missing required badge: {badge}")]
    MissingBadge { badge: String },

    #[error("Cannot claim your own task")]
    CannotClaimOwnTask {},
}
