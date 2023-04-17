use super::token::Token;
use crate::Request;
use anyhow::anyhow;
use std::collections::HashMap;

pub(super) struct Parser {
    env: HashMap<String, String>,
}

impl Parser {
    pub fn new(env: HashMap<String, String>) -> Self {
        Self { env }
    }

    pub fn execute(&mut self, tokens: Vec<Token>) -> anyhow::Result<Request> {
        let command = tokens.first().ok_or(anyhow!("Command not specified"))?;

        match command.value.as_str() {
            "set" => self.parse_set(tokens),
            "get" => self.parse_get(tokens),
            "delete" => self.parse_delete(tokens),
            "exists" => self.parse_exists(tokens),
            "increment" => self.parse_increment(tokens),
            "decrement" => self.parse_decrement(tokens),
            "flush" => self.parse_flush(tokens),
            _ => Err(anyhow!(
                "Command not found (line {}, column {})",
                command.line,
                command.column
            )),
        }
    }

    fn fetch_env(&self, value: &str) -> String {
        let mut chars = value.chars();

        if value.starts_with("$") {
            chars.next();

            if let Some(value) = self.env.get(chars.as_str()) {
                return value.into();
            }
        }

        value.into()
    }

    fn parse_set(&mut self, tokens: Vec<Token>) -> anyhow::Result<Request> {
        if tokens.len() != 3 {
            return Err(anyhow!(
                "Set command requires two (2) arguments (line {}, column {})",
                tokens[0].line,
                tokens[0].column
            ));
        }

        let (key, value) = (&tokens[1], &tokens[2]);
        let (key, value) = (self.fetch_env(&key.value), self.fetch_env(&value.value));

        Ok(Request::Set(key, value))
    }

    fn parse_get(&mut self, tokens: Vec<Token>) -> anyhow::Result<Request> {
        if tokens.len() != 2 {
            return Err(anyhow!(
                "Get command requires one (1) argument (line {}, column {})",
                tokens[0].line,
                tokens[0].column
            ));
        }

        let key = self.fetch_env(&tokens[1].value);
        Ok(Request::Get(key))
    }

    fn parse_delete(&mut self, tokens: Vec<Token>) -> anyhow::Result<Request> {
        if tokens.len() != 2 {
            return Err(anyhow!(
                "Delete command requires one (1) argument (line {}, column {})",
                tokens[0].line,
                tokens[0].column
            ));
        }

        let key = self.fetch_env(&tokens[1].value);
        Ok(Request::Delete(key))
    }

    fn parse_exists(&mut self, tokens: Vec<Token>) -> anyhow::Result<Request> {
        if tokens.len() != 2 {
            return Err(anyhow!(
                "Exists command requires one (1) argument (line {}, column {})",
                tokens[0].line,
                tokens[0].column
            ));
        }

        let key = self.fetch_env(&tokens[1].value);
        Ok(Request::Exists(key))
    }

    fn parse_increment(&mut self, tokens: Vec<Token>) -> anyhow::Result<Request> {
        if tokens.len() != 2 {
            return Err(anyhow!(
                "Increment command requires two (2) arguments (line {}, column {})",
                tokens[0].line,
                tokens[0].column
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
            Err(anyhow!(
                "Second argument for increment command must be a valid unsigned integer (line {}, column {})",
                value_token.line,
                value_token.column
            ))
        }
    }

    fn parse_decrement(&mut self, tokens: Vec<Token>) -> anyhow::Result<Request> {
        if tokens.len() != 2 {
            return Err(anyhow!(
                "Decrement command requires two (2) arguments (line {}, column {})",
                tokens[0].line,
                tokens[0].column
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
            Err(anyhow!(
                "Second argument for decrement command must be a valid unsigned integer (line {}, column {})",
                value_token.line,
                value_token.column
            ))
        }
    }

    fn parse_flush(&mut self, tokens: Vec<Token>) -> anyhow::Result<Request> {
        if tokens.len() != 1 {
            return Err(anyhow!(
                "Flush command can't take any value (line {}, column {})",
                tokens[0].line,
                tokens[0].column
            ));
        }

        Ok(Request::Flush)
    }
}
