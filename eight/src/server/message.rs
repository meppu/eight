use tokio::sync::oneshot;

#[derive(Debug)]
pub enum Request {
    Set(String, String),
    Get(String),
    Delete(String),
    Flush,
}

#[derive(Debug)]
pub(crate) struct ServerRequest {
    pub(crate) sender: oneshot::Sender<Response>,
    pub(crate) request: Request,
}

#[derive(Debug)]
pub enum Response {
    Ok,
    Value(String),
    Error(anyhow::Error),
}

impl Response {
    pub fn result(self) -> anyhow::Result<Response> {
        match self {
            Response::Error(err) => Err(err),
            other => Ok(other),
        }
    }

    pub fn option(self) -> Option<Response> {
        self.result().ok()
    }
}
