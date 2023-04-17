use super::{lexer::lex, parser::Parser};
use crate::{Response, Server};
use anyhow::anyhow;
use std::{collections::HashMap, mem};

#[derive(Debug, Default, Clone)]
pub struct QueryExecutor {
    source: String,
    env: HashMap<String, String>,
}

impl QueryExecutor {
    pub fn new(source: String, env: HashMap<String, String>) -> Self {
        Self { source, env }
    }

    pub async fn execute(&mut self, server: &Server) -> anyhow::Result<Response> {
        let collection = lex(mem::take(&mut self.source));
        let mut parser = Parser::new(mem::take(&mut self.env));

        let mut last_result = Err(anyhow!("Nothing to execute"));
        for tokens in collection {
            let command = parser.execute(tokens)?;
            last_result = server.call(command).await;
        }

        last_result
    }
}
