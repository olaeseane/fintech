use warp::reject::Reject;

/// An application-specific error type
#[derive(Debug, PartialEq, Eq)]
pub enum ApplicationError {
    /// Account wasn't found
    AccountNotFound(String),

    /// Not enough currency in the account (underflow)
    AccountUnderFunded(String, u64),

    /// Too much currency in the account (overflow)
    AccountOverFunded(String, u64),
}

#[derive(Debug)]
pub struct OctopusError(pub ApplicationError);

impl Reject for OctopusError {}
