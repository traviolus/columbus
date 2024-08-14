use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Duplicate island")]
    Duplicate {},

    #[error("Island not found")]
    NotFound {},

    #[error("You can only gather resources once per minute")]
    GatherLimit {},
}
