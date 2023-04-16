use anyhow::anyhow;
use tokio::sync::oneshot;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum Response {
    Ok,
    Value(String),
    Error(&'static str),
}

impl Response {
    pub fn result(self) -> anyhow::Result<Response> {
        match self {
            Response::Error(err) => Err(anyhow!(err)),
            other => Ok(other),
        }
    }

    pub fn option(self) -> Option<Response> {
        self.result().ok()
    }
}
