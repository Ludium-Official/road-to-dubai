use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Name has been taken (name {name})")]
    NameTaken { name: String },

    #[error("Insufficient funds sent")]
    InsufficientFundsSend {},

    #[error("Name too short (length {length} min_length {min_length})")]
    NameTooShort { length: u64, min_length: u64 },

    #[error("Name too long (length {length} min_length {max_length})")]
    NameTooLong { length: u64, max_length: u64 },

    #[error("Invalid character(char {c}")]
    InvalidCharacter { c: char },

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Name does not exist (name {name})")]
    NameNotExists { name: String },
}