use crate::Response;
use thiserror::Error;

/// Short version of [`std::result::Result<T, self::Error>`]
pub type Result<T> = std::result::Result<T, self::Error>;

/// Custom error type for eight.
#[derive(Error, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type", content = "value"))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Error {
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
    #[error("You don't have permission to perform this operation")]
    PermissionFailure,
}

impl Error {
    /// Turns [`enum@Error`] into [`Response::Error`]
    pub fn as_response(&self) -> Response {
        Response::Error(self.clone())
    }
}

macro_rules! err {
    ($name:ident) => {
        crate::Error::$name
    };

    ($fmt:expr, $token:expr) => {
        crate::Error::CommandError($fmt.to_string(), $token.line, $token.column)
    };
}

pub(crate) use err;
