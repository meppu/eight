use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Serialize, Deserialize)]
pub struct Request {
    #[serde(default)]
    pub id: String,
    pub query: String,
    pub vars: HashMap<String, String>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub id: String,
    pub results: Vec<crate::embedded::messaging::Response>,
}

#[derive(Default)]
pub struct QueryBuilder {
    req: Request,
}

impl QueryBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from_id<T: ToString>(id: T) -> Self {
        Self {
            req: Request {
                id: id.to_string(),
                ..Default::default()
            },
        }
    }

    pub fn bind<T: ToString>(mut self, key: T, value: T) -> Self {
        self.req.vars.insert(key.to_string(), value.to_string());
        self
    }

    pub fn set_id<T: ToString>(mut self, id: T) -> Self {
        self.req.id = id.to_string();
        self
    }

    pub fn set_query(mut self, query: String) -> Self {
        self.req.query = query;
        self
    }

    pub fn add_query(mut self, query: &str) -> Self {
        self.req.query.push_str(query);
        self.req.query.push('\n');

        self
    }

    pub fn collect(self) -> Request {
        self.req
    }
}
