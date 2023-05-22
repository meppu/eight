use super::token::Token;
use crate::{
    embedded::{messaging::Request, Error, Result},
    err,
};
use std::collections::HashMap;

pub(super) struct Parser {
    env: HashMap<String, String>,
}

#[derive(Debug, PartialEq)]
pub(super) enum CallType {
    Await(Request),
    Spawn(Request),
}

impl Parser {
    pub fn new(env: HashMap<String, String>) -> Self {
        Self { env }
    }

    pub fn execute(&mut self, tokens: Vec<Token>) -> Result<CallType> {
        let command = tokens.first().ok_or(Error::CommandNotFound)?;
        let mut command_name = command.value.chars();

        // check if call or cast
        let is_cast = if command.value.ends_with('?') {
            command_name.next_back();

            true
        } else {
            false
        };

        // manually implemented to prevent string allocation for every single query.
        // also for only accepting full uppercase. SeT or ExisTs is not a valid command.
        let request = match command_name.as_str() {
            "set" | "SET" => self.parse_set(tokens),
            "get" | "GET" => self.parse_get(tokens),
            "delete" | "DELETE" => self.parse_delete(tokens),
            "exists" | "EXISTS" => self.parse_exists(tokens),
            "incr" | "INCR" => self.parse_increment(tokens),
            "decr" | "DECR" => self.parse_decrement(tokens),
            "search" | "SEARCH" => self.parse_search(tokens),
            "flush" | "FLUSH" => self.parse_flush(tokens),
            "downgrade" | "DOWNGRADE" => self.parse_downgrade(tokens),
            _ => Err(err!("Command not found", command)),
        }?;

        Ok(if is_cast {
            CallType::Spawn(request)
        } else {
            CallType::Await(request)
        })
    }

    fn fetch_env(&self, value: &str) -> String {
        let mut chars = value.chars();

        // $ stands for variable
        if value.starts_with('$') {
            chars.next();

            if let Some(value) = self.env.get(chars.as_str()) {
                return value.to_owned();
            }
        }

        value.to_string()
    }

    fn parse_set(&mut self, tokens: Vec<Token>) -> Result<Request> {
        if tokens.len() != 3 {
            Err(err!("Set command requires two (2) argument", tokens[0]))
        } else {
            let (key, value) = (&tokens[1], &tokens[2]);
            let (key, value) = (self.fetch_env(&key.value), self.fetch_env(&value.value));

            Ok(Request::Set(key, value))
        }
    }

    fn parse_get(&mut self, tokens: Vec<Token>) -> Result<Request> {
        if tokens.len() != 2 {
            Err(err!("Get command requires one (1) argument", tokens[0]))
        } else {
            let key = self.fetch_env(&tokens[1].value);
            Ok(Request::Get(key))
        }
    }

    fn parse_delete(&mut self, tokens: Vec<Token>) -> Result<Request> {
        if tokens.len() != 2 {
            Err(err!("Delete command requires one (1) argument", tokens[0]))
        } else {
            let key = self.fetch_env(&tokens[1].value);
            Ok(Request::Delete(key))
        }
    }

    fn parse_exists(&mut self, tokens: Vec<Token>) -> Result<Request> {
        if tokens.len() != 2 {
            Err(err!("Exists command requires one (1) argument", tokens[0]))
        } else {
            let key = self.fetch_env(&tokens[1].value);
            Ok(Request::Exists(key))
        }
    }

    fn parse_increment(&mut self, tokens: Vec<Token>) -> Result<Request> {
        if tokens.len() != 3 {
            return Err(err!(
                "Increment command requires two (2) argument",
                tokens[0]
            ));
        }

        let (key_token, value_token) = (&tokens[1], &tokens[2]);
        let (key, value) = (
            self.fetch_env(&key_token.value),
            self.fetch_env(&value_token.value),
        );

        let number = value.parse::<usize>().map_err(|_| {
            err!(
                "Second argument for increment command must be a valid unsigned integer",
                value_token
            )
        })?;

        Ok(Request::Increment(key, number))
    }

    fn parse_decrement(&mut self, tokens: Vec<Token>) -> Result<Request> {
        if tokens.len() != 3 {
            return Err(err!(
                "Decrement command requires two (2) argument",
                tokens[0]
            ));
        }

        let (key_token, value_token) = (&tokens[1], &tokens[2]);
        let (key, value) = (
            self.fetch_env(&key_token.value),
            self.fetch_env(&value_token.value),
        );

        let number = value.parse::<usize>().map_err(|_| {
            err!(
                "Second argument for decrement command must be a valid unsigned integer",
                value_token
            )
        })?;

        Ok(Request::Decrement(key, number))
    }

    fn parse_search(&mut self, tokens: Vec<Token>) -> Result<Request> {
        if tokens.len() != 2 {
            Err(err!("Search command requires one (1) argument", tokens[0]))
        } else {
            let key = self.fetch_env(&tokens[1].value);
            Ok(Request::Search(key))
        }
    }

    fn parse_flush(&mut self, tokens: Vec<Token>) -> Result<Request> {
        if tokens.len() != 1 {
            Err(err!("Flush command can't take any value", tokens[0]))
        } else {
            Ok(Request::Flush)
        }
    }

    fn parse_downgrade(&mut self, tokens: Vec<Token>) -> Result<Request> {
        if tokens.len() != 1 {
            Err(err!(
                "Downgrade permission command can't take any value",
                tokens[0]
            ))
        } else {
            Ok(Request::DowngradePermission)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn tokenize(input: &str) -> Vec<Token> {
        input
            .split(' ')
            .map(|t| Token {
                value: t.to_string(),
                line: 1,
                column: 1,
            })
            .collect()
    }

    #[test]
    fn test_execute() {
        let mut env = HashMap::new();

        let a = "A".to_string();
        let b = "B".to_string();
        let c = 1;

        env.insert("varA".to_string(), a.clone());
        env.insert("varB".to_string(), b.clone());
        env.insert("varC".to_string(), c.to_string());
        let mut parser = Parser::new(env);

        assert_eq!(
            parser.execute(tokenize("set $varA $varB")).unwrap(),
            CallType::Await(Request::Set(a.clone(), b.clone()))
        );
        assert_eq!(
            parser.execute(tokenize("get $varA")).unwrap(),
            CallType::Await(Request::Get(a.clone()))
        );
        assert_eq!(
            parser.execute(tokenize("delete $varA")).unwrap(),
            CallType::Await(Request::Delete(a.clone()))
        );
        assert_eq!(
            parser.execute(tokenize("incr $varA $varC")).unwrap(),
            CallType::Await(Request::Increment(a.clone(), c))
        );
        assert_eq!(
            parser.execute(tokenize("decr $varA $varC")).unwrap(),
            CallType::Await(Request::Decrement(a.clone(), c))
        );
        assert_eq!(
            parser.execute(tokenize("search $varA")).unwrap(),
            CallType::Await(Request::Search(a.clone()))
        );
        assert_eq!(
            parser.execute(tokenize("flush")).unwrap(),
            CallType::Await(Request::Flush)
        );
        assert_eq!(
            parser.execute(tokenize("downgrade")).unwrap(),
            CallType::Await(Request::DowngradePermission)
        );

        assert_eq!(
            parser.execute(tokenize("set? $varA $varB")).unwrap(),
            CallType::Spawn(Request::Set(a.clone(), b.clone()))
        );
    }
}
