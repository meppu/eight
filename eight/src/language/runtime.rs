use super::{
    lexer::lex,
    parser::{CallType, Parser},
};
use crate::{EightResult, Response, Server};
use std::{collections::HashMap, mem};

#[derive(Debug, Default, Clone)]
pub(crate) struct QueryExecutor {
    source: String,
    env: HashMap<String, String>,
}

impl QueryExecutor {
    pub fn new(source: String, env: HashMap<String, String>) -> Self {
        Self { source, env }
    }

    pub async fn execute(&mut self, server: &Server) -> EightResult<Vec<Response>> {
        let collection = lex(mem::take(&mut self.source));
        let mut parser = Parser::new(mem::take(&mut self.env));
        let mut results = Vec::new();

        for tokens in collection {
            let command = parser.execute(tokens)?;

            match command {
                CallType::Await(request) => {
                    results.push(server.call(request).await?);
                }
                CallType::Spawn(request) => {
                    server.cast(request).await?;
                }
            }
        }

        Ok(results)
    }
}
