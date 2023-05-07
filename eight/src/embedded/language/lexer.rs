use super::token::Token;
use std::mem;

#[derive(Default, PartialEq)]
enum State {
    String,
    #[default]
    Identifier,
    Comment,
}

#[derive(Default)]
pub(super) struct Lexer {
    source: String,
    line: usize,
    column: usize,
    state: State,
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
            match self.state {
                State::String => self.handle_string(character),
                State::Identifier => self.handle_identifier(character),
                State::Comment => self.handle_comment(character),
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

    fn handle_string(&mut self, character: char) {
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

    fn handle_identifier(&mut self, character: char) {
        match character {
            ' ' | '\t' | '\n' | '\r' => self.make_token(),
            ';' => {
                self.make_token();

                self.temp = ";".to_string();
                self.make_token();
            }
            '"' => self.state = State::String,
            '#' => self.state = State::Comment,
            other => self.temp.push(other),
        }
    }

    fn handle_comment(&mut self, character: char) {
        if character == '\n' {
            self.state = State::default();
        }
    }

    fn make_token(&mut self) {
        let old_state = mem::take(&mut self.state);
        if self.temp.is_empty() && old_state != State::String {
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
