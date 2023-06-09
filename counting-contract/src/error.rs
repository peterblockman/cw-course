use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError), // this convert between Error and StdError

    #[error("Unauthorized - only {owner} can call it")]
    Unauthorized { owner: String}
}