use crate::token::{Token, TokenType};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Lexer {
    pub input: String,
    pub position: usize,      // current posistion in input (points to curr char)
    pub read_position: usize, // current reading position in input (after current char)
    pub ch: char,             // current char under examination
}

impl Lexer {
    pub fn new(_input: &str) -> Lexer {
        let l = Lexer {
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
        let mut token = Token::new();

        // TODO: implement this
        // self.skip_whitespace();

        // checking for double character tokens such as ==, !=, <=, <> etc...
        if self.peek_char() == '=' {
            let placeholder_ch = self.ch;
            self.read_char();
            let literal = placeholder_ch.to_string() + &self.ch.to_string();
            token = Token {
                token_type: TokenType::Eq,
                literal: literal,
            };
        }

        return token;
    }

    // pub fn skip_whitespace(&self) {
    //     for self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
    //         self.read_char();
    //     }
    // }

    pub fn read_char(&mut self) {
        if self.position > self.input.chars().count() {
            self.ch = '\0';
        } else {
            self.ch = self.input.as_bytes()[self.position] as char;
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn peek_char(&mut self) -> char {
        if self.read_position > self.input.chars().count() {
            return '\0';
        } else {
            return self.input.as_bytes()[self.read_position] as char;
        }
    }

    pub fn read_identifier(&mut self) -> String {
        let position: usize = self.position;
        while self.ch.is_alphabetic() {
            self.read_char();
        }

        // return all chars between position and self.position as a string
        return self.input[position..self.position].to_string();
    }

    pub fn read_number(&mut self) -> String {
        let position: usize = self.position;
        while self.ch.is_digit(10) {
            self.read_char();
        }
        return self.input[position..self.position].to_string();
    }
}
