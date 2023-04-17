use super::token::Token;
use std::mem;

#[derive(Debug, Default, Clone)]
pub(super) struct Lexer {
    source: String,
    line: usize,
    column: usize,
    state: bool,
    tokens: Vec<Token>,
    temp: String,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source,
            line: 1,
            ..Default::default()
        }
    }

    pub fn execute(&mut self) {
        let source = mem::take(&mut self.source);

        for character in source.chars() {
            if self.state {
                self.collect_string(character);
            } else {
                self.collect_identifier(character);
            }

            if character == '\n' {
                self.line += 1;
                self.column = 0;
            } else {
                self.column += 1;
            }
        }

        self.make_token();
    }

    pub fn collect(&mut self) -> Vec<Vec<Token>> {
        let tokens = mem::take(&mut self.tokens);

        let mut new_tokens = Vec::new();
        let mut temporary = Vec::new();

        for token in tokens {
            if &token.value == ";" {
                new_tokens.push(mem::take(&mut temporary));
            } else {
                temporary.push(token);
            }
        }

        new_tokens
    }

    fn collect_string(&mut self, character: char) {
        if character == '"' {
            match self.temp.pop() {
                Some('\\') => self.temp.push(character),
                Some(value) => {
                    self.temp.push(value);
                    self.make_token();
                }
                None => self.make_token(),
            }
        } else {
            self.temp.push(character);
        }
    }

    fn collect_identifier(&mut self, character: char) {
        match character {
            ' ' | '\t' | '\n' | '\r' => self.make_token(),
            ';' => {
                self.make_token();

                self.temp = ";".to_string();
                self.make_token();
            }
            '"' => self.state = true,
            other => self.temp.push(other),
        }
    }

    fn make_token(&mut self) {
        self.state = false;

        if self.temp.is_empty() {
            return;
        }

        let token = Token {
            value: mem::take(&mut self.temp),
            line: self.line,
            column: self.column,
        };

        self.tokens.push(token);
    }
}

pub(super) fn lex(source: String) -> Vec<Vec<Token>> {
    let mut lexer = Lexer::new(source);
    lexer.execute();

    lexer.collect()
}
