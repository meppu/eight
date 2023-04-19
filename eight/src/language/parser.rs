use super::token::Token;
use crate::Request;
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

    pub fn execute(&mut self, tokens: Vec<Token>) -> crate::Result<CallType> {
        let command = tokens.first().ok_or(crate::Error::CommandNotFound)?;
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
            "inc" => self.parse_increment(tokens),
            "dec" => self.parse_decrement(tokens),
            "flush" => self.parse_flush(tokens),
            _ => Err(crate::Error::CommandError(
                "Command not found".into(),
                command.line,
                command.column,
            )),
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
                return value.into();
            }
        }

        value.into()
    }

    fn parse_set(&mut self, tokens: Vec<Token>) -> crate::Result<Request> {
        if tokens.len() != 3 {
            return Err(crate::Error::CommandError(
                "Set command requires two (2) argument".into(),
                tokens[0].line,
                tokens[0].column,
            ));
        }

        let (key, value) = (&tokens[1], &tokens[2]);
        let (key, value) = (self.fetch_env(&key.value), self.fetch_env(&value.value));

        Ok(Request::Set(key, value))
    }

    fn parse_get(&mut self, tokens: Vec<Token>) -> crate::Result<Request> {
        if tokens.len() != 2 {
            return Err(crate::Error::CommandError(
                "Get command requires one (1) argument".into(),
                tokens[0].line,
                tokens[0].column,
            ));
        }

        let key = self.fetch_env(&tokens[1].value);
        Ok(Request::Get(key))
    }

    fn parse_delete(&mut self, tokens: Vec<Token>) -> crate::Result<Request> {
        if tokens.len() != 2 {
            return Err(crate::Error::CommandError(
                "Delete command requires one (1) argument".into(),
                tokens[0].line,
                tokens[0].column,
            ));
        }

        let key = self.fetch_env(&tokens[1].value);
        Ok(Request::Delete(key))
    }

    fn parse_exists(&mut self, tokens: Vec<Token>) -> crate::Result<Request> {
        if tokens.len() != 2 {
            return Err(crate::Error::CommandError(
                "Exists command requires one (1) argument".into(),
                tokens[0].line,
                tokens[0].column,
            ));
        }

        let key = self.fetch_env(&tokens[1].value);
        Ok(Request::Exists(key))
    }

    fn parse_increment(&mut self, tokens: Vec<Token>) -> crate::Result<Request> {
        if tokens.len() != 3 {
            return Err(crate::Error::CommandError(
                "Increment command requires two (2) argument".into(),
                tokens[0].line,
                tokens[0].column,
            ));
        }

        let (key_token, value_token) = (&tokens[1], &tokens[2]);
        let (key, value) = (
            self.fetch_env(&key_token.value),
            self.fetch_env(&value_token.value),
        );

        if let Ok(number) = value.parse::<usize>() {
            Ok(Request::Increment(key, number))
        } else {
            Err(crate::Error::CommandError(
                "Second argument for increment command must be a valid unsigned integer".into(),
                value_token.line,
                value_token.column,
            ))
        }
    }

    fn parse_decrement(&mut self, tokens: Vec<Token>) -> crate::Result<Request> {
        if tokens.len() != 3 {
            return Err(crate::Error::CommandError(
                "Decrement command requires two (2) argument".into(),
                tokens[0].line,
                tokens[0].column,
            ));
        }

        let (key_token, value_token) = (&tokens[1], &tokens[2]);
        let (key, value) = (
            self.fetch_env(&key_token.value),
            self.fetch_env(&value_token.value),
        );

        if let Ok(number) = value.parse::<usize>() {
            Ok(Request::Decrement(key, number))
        } else {
            Err(crate::Error::CommandError(
                "Second argument for decrement command must be a valid unsigned integer".into(),
                value_token.line,
                value_token.column,
            ))
        }
    }

    fn parse_flush(&mut self, tokens: Vec<Token>) -> crate::Result<Request> {
        if tokens.len() != 1 {
            return Err(crate::Error::CommandError(
                "Flush command can't take any value".into(),
                tokens[0].line,
                tokens[0].column,
            ));
        }

        Ok(Request::Flush)
    }
}
