use crate::Response;
use thiserror::Error;

/// Short version of [`Result<T, EightError>`]
pub type EightResult<T> = Result<T, EightError>;

/// Custom error type for eight.
#[derive(Error, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum EightError {
    #[error("Key length must be longer than two (2) characters")]
    KeyTooShort,
    #[error("Key must be a valid alphanumeric character")]
    KeyWrongFormat,
    #[error("Unknown error while checking key")]
    CheckExistsFail,
    #[error("Unknown error while creating key")]
    CreateDirFail,
    #[error("Setting key failed (probably invalid key)")]
    FileWriteFail,
    #[error("Getting key failed (probably invalid key)")]
    FileReadFail,
    #[error("Deleting key failed (probably invalid key)")]
    FileRemoveFail,
    #[error("Removing a directory failed due to filesystem error")]
    DirRemoveFail,
    #[error("Value must be a valid unsigned integer")]
    UIntParseFail,
    #[error("Sending message failed")]
    SendFail,
    #[error("Receive message failed")]
    RecvFail,
    #[error("Receive message timeout")]
    RecvTimeout,
    #[error("Nothing to execute")]
    CommandNotFound,
    #[error("{0} (line {1}, column {2})")]
    CommandError(String, usize, usize),
}

impl EightError {
    /// Turns [`EightError`] into [`Response::Error`]
    pub fn as_response(&self) -> Response {
        Response::Error(self.to_string())
    }
}
