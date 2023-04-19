use tokio::sync::oneshot;

#[derive(Debug)]
pub enum Request {
    Set(String, String),
    Get(String),
    Delete(String),
    Exists(String),
    Increment(String, usize),
    Decrement(String, usize),
    Flush,
}

#[derive(Debug)]
pub(super) struct ServerRequest {
    pub(super) sender: oneshot::Sender<Response>,
    pub(super) request: Request,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Response {
    Ok,
    Value(String),
    Error(String),
}
