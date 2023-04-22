use eight::Response;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type QueryVariables = HashMap<String, String>;

#[derive(Deserialize)]
pub struct QueryRequest {
    #[serde(default)]
    pub id: String,
    pub query: String,
    pub vars: QueryVariables,
}

#[derive(Serialize)]
pub struct QueryResponse {
    #[serde(default)]
    pub id: String,
    pub results: Vec<Response>,
}
