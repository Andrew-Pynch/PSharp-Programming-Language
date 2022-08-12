use crate::token::{Token, TokenType};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Lexer {
    pub input: String,
    pub position: i32,      // current posistion in input (points to curr char)
    pub read_position: i32, // current reading position in input (after current char)
    pub ch: char,           // current char under examination
}

impl Lexer {
    pub fn new(_input: &str) -> Lexer {
        let mut l = Lexer {
            input: _input.to_string(),
            position: 0,
            read_position: 0,
            ch: _input.as_bytes()[0] as char,
        };

        // TODO: Implement this
        // l.read_char()

        return l;
    }

    pub fn next_token(&mut self) -> Token {
        let token = Token::new();

        // TODO: implement this
        // self.skip_whitespace();

        // checking for double character tokens such as ==, !=, <=, <> etc...
        if (self.peekChar() == "=") {
            let placeholder_ch = self.ch;
            self.read_char();
            let literal = placeholder_ch.to_string() + self.ch.to_string();
            let token: Token = Token {
                token_type: TokenType::Eq,
                literal: literal,
            };
        }

        return token;
    }
}
