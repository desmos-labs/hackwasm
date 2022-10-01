use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Must set version to `{version}")]
    InvalidIbcVersion { version: String },

    #[error("Counter party must set version to `{version}")]
    InvalidCounterPartyIbcVersion { version: String },

    #[error("Only supports unordered channel")]
    OnlyOrderedChannel {},
}
