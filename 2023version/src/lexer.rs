use std::fmt;

use crate::token::{self, Token, TokenType};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Lexer {
    pub input: String,
    pub position: usize, // points to char in input that corresponds to ch byte
    pub read_position: usize, // points to next char in input
    pub ch: char,
}

impl std::fmt::Display for Lexer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Lexer {{\n  input: \"{}\",\n  position: {},\n  read_position: {},\n  ch: '{}'\n}}",
            self.input, self.position, self.read_position, self.ch
        )
    }
}

impl Lexer {
    pub fn new(_input: &str) -> Lexer {
        let mut l: Lexer = Lexer {
            input: _input.to_string(),
            position: 0,
            read_position: 0,
            ch: _input.as_bytes()[0] as char,
        };

        return l;
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0'; // Use the null character '\0' to represent the end of input
        } else {
            self.ch = self.input.as_bytes()[self.read_position] as char;
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        if self.position > 0 {
            self.read_char();
        }

        self.skip_whitespace();

        let isLetter = is_letter(self.ch);
        let isDigit = is_digit(self.ch);

        if is_letter(self.ch) {
            let literal = self.read_identifier();
            let res: Token = Token::new(token::lookup_ident(&literal), literal);

            return res;
        } else if is_digit(self.ch) {
            let number = self.read_number();
            let res: Token = Token::new(TokenType::INT, number);

            return res;
        } else {
            let res: Token = match self.ch {
                '=' => Token::new(TokenType::ASSIGN, self.ch.to_string()),
                ';' => Token::new(TokenType::SEMICOLON, self.ch.to_string()),
                '(' => Token::new(TokenType::LPAREN, self.ch.to_string()),
                ')' => Token::new(TokenType::RPAREN, self.ch.to_string()),
                ',' => Token::new(TokenType::COMMA, self.ch.to_string()),
                '+' => Token::new(TokenType::PLUS, self.ch.to_string()),
                '{' => Token::new(TokenType::LBRACE, self.ch.to_string()),
                '}' => Token::new(TokenType::RBRACE, self.ch.to_string()),
                '\0' => Token::new(TokenType::EOF, self.ch.to_string()),
                _ => Token::new(TokenType::ILLEGAL, self.ch.to_string()),
            };

            return res;
        };
    }

    pub fn read_identifier(&mut self) -> String {
        let position: usize = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }
        // just packing this in intermediate result so I can debug
        let result: String = self.input[position..self.position].to_string();
        dbg!(result.clone());
        return result;
    }

    pub fn read_number(&mut self) -> String {
        let position: usize = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        return self.input[position..self.position].to_string();
    }

    pub fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }
}

pub fn is_letter(ch: char) -> bool {
    return 'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_';
}

pub fn is_digit(ch: char) -> bool {
    return '0' <= ch && ch <= '9';
}
