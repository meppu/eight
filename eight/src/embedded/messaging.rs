/// Allows you to send request to server.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type", content = "value"))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Request {
    /// Set request with key and value. Returns [`Response::Ok`] on success.
    Set(String, String),
    /// Get request with key. Returns [`Response::Text`] on success.
    Get(String),
    /// Delete request with key. Returns [`Response::Ok`] on success.
    Delete(String),
    /// Exists request with key. Returns [`Response::Boolean`] on success.
    Exists(String),
    /// Increment request with key and increment value. Returns [`Response::Number`] on success.
    Increment(String, usize),
    /// Decrement request with key and decrement value. Returns [`Response::Number`] on success.
    Decrement(String, usize),
    /// Search key. Returns [`Response::TextList`] on success.
    Search(String),
    /// Flush request. Returns [`Response::Ok`] on success.
    Flush,
    /// Downgrade permission. Returns [`Response::Ok`] on success.
    DowngradePermission,
}

/// Allows you to get response from server.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type", content = "value"))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Response {
    /// Success, no value returned from server.
    Ok,
    /// Success, with text returned from server.
    Text(String),
    /// Success, with number returned from server.
    Number(usize),
    /// Success, with bool returned from server.
    Boolean(bool),
    /// Success, with text list returned from server.
    TextList(Vec<String>),
    /// Error, with error value returned from server.
    Error(crate::embedded::Error),
}
