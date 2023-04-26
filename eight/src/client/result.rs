use thiserror::Error;

/// Short version of [`std::result::Result<T, self::Error>`]
pub type Result<T> = std::result::Result<T, self::Error>;

/// Custom error type for eight client.
#[derive(Error, Debug, Clone, PartialEq)]
pub enum Error {
    #[error("Sending HTTP Request failed")]
    HTTPRequestFail,
    #[error("Reading body content failed")]
    ReadBodyFail,
}
