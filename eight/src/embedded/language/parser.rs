use super::token::Token;
use crate::{
    embedded::{messaging::Request, Error, Result},
    err,
};
use std::collections::HashMap;

#[derive(Debug)]
pub(super) struct Parser {
    env: HashMap<String, String>,
}

#[derive(Debug)]
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

        let request = match command_name.as_str() {
            "set" => self.parse_set(tokens),
            "get" => self.parse_get(tokens),
            "delete" => self.parse_delete(tokens),
            "exists" => self.parse_exists(tokens),
            "incr" => self.parse_increment(tokens),
            "decr" => self.parse_decrement(tokens),
            "search" => self.parse_search(tokens),
            "flush" => self.parse_flush(tokens),
            "downgrade" => self.parse_downgrade(tokens),
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
