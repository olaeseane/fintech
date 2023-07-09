use thiserror::Error;
use url::ParseError;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("not a number")]
    InvalidNumber(),

    #[error("invalid order parameters")]
    InvalidOrderParameters(String),

    #[error("reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error(transparent)]
    ParseError(#[from] ParseError),

    #[error("{0}")]
    LogicError(String),
}
