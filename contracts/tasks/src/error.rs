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

    #[error("Task is not in submitted status")]
    TaskNotSubmitted {},

    #[error("Task has not expired")]
    TaskNotExpired {},

    #[error("Agent not registered in reputation contract")]
    AgentNotRegistered {},

    #[error("Cannot claim your own task")]
    CannotClaimOwnTask {},

    #[error("Must send exactly one coin denomination (uxion)")]
    InvalidFunds {},

    #[error("Escrow amount {sent} is below minimum {minimum}")]
    EscrowTooLow { sent: String, minimum: String },

    #[error("Wrong denomination: expected {expected}, got {got}")]
    WrongDenom { expected: String, got: String },

    #[error("Task cannot be cancelled (already claimed or completed)")]
    CannotCancel {},

    #[error("Auto-release not yet available, {remaining} seconds remain")]
    AutoReleaseNotReady { remaining: u64 },
}
