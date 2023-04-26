//! Types for messaging between web server.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Request struct. This is literaly what you send as JSON while using client.
#[derive(Default, Serialize, Deserialize)]
pub struct Request {
    #[serde(default)]
    pub id: String,
    pub query: String,
    pub vars: HashMap<String, String>,
}
/// Response struct. This is literaly what you receive as JSON while using client.

#[derive(Default, Serialize, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub id: String,
    pub results: Vec<crate::embedded::messaging::Response>,
}

/// Flexible query builder for [`Request`].
///
/// ```no_run
/// use eight::client::messaging::QueryBuilder;
///
/// let request = QueryBuilder::new()
///   .add_query("set $user 0")
///   .bind("user", "bob")
///   .set_id("some_unique_id_to_handle")
///   .collect();
///
/// ```
#[derive(Default)]
pub struct QueryBuilder {
    req: Request,
}

impl QueryBuilder {
    /// Create new query builder.
    ///
    /// This function is same with [`Default::default`].
    pub fn new() -> Self {
        Default::default()
    }

    /// Create query builder with given id.
    pub fn from_id<T: ToString>(id: T) -> Self {
        Self {
            req: Request {
                id: id.to_string(),
                ..Default::default()
            },
        }
    }

    /// Add an environment variable to query.
    pub fn bind<T: ToString>(mut self, key: T, value: T) -> Self {
        self.req.vars.insert(key.to_string(), value.to_string());
        self
    }

    /// Set request id. Which is used for asynchronous message handling.
    pub fn set_id<T: ToString>(mut self, id: T) -> Self {
        self.req.id = id.to_string();
        self
    }

    /// Set query. This function removes old values.
    pub fn set_query(mut self, query: String) -> Self {
        self.req.query = query;
        self
    }

    /// Append a query.
    pub fn add_query(mut self, query: &str) -> Self {
        self.req.query.push_str(query);
        self.req.query.push('\n');

        self
    }

    /// Collect [`Request`] result.
    pub fn collect(self) -> Request {
        self.req
    }
}
