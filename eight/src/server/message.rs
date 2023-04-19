use tokio::sync::oneshot;

/// Allows you to send request to server.
#[derive(Debug, Clone, PartialEq)]
pub enum Request {
    /// Set request with key and value.
    Set(String, String),
    /// Get request with key.
    Get(String),
    /// Delete request with key.
    Delete(String),
    /// Exists request with key.
    Exists(String),
    /// Increment request with key and increment value.
    Increment(String, usize),
    /// Decrement request with key and decrement value.
    Decrement(String, usize),
    /// Flush request.
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
    /// Success, with value returned from server.
    Value(String),
    /// Error, with description returned from server.
    Error(String),
}
