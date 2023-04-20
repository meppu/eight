use tokio::sync::oneshot;

/// Allows you to send request to server.
#[derive(Debug, Clone, PartialEq)]
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
    /// Flush request. Returns [`Response::Ok`] on success.
    Flush,
}

#[derive(Debug)]
pub(super) struct ServerRequest {
    pub(super) sender: oneshot::Sender<Response>,
    pub(super) request: Request,
}

/// Allows you to get response from server.
#[derive(Debug, Clone, PartialEq)]
pub enum Response {
    /// Success, no value returned from server.
    Ok,
    /// Success, with text returned from server.
    Text(String),
    /// Success, with number returned from server.
    Number(usize),
    /// Success, with bool returned from server.
    Boolean(bool),
    /// Success, with list returned from server.
    List(Vec<String>),
    /// Error, with description returned from server.
    Error(String),
}
