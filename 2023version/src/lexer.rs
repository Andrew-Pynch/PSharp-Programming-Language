use crate::token::{self, Token, TokenType};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Lexer {
    pub input: String,
    pub position: usize, // points to char in input that corresponds to ch byte
    pub read_position: usize, // points to next char in input
    pub ch: char,
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
        let tok: Token;

        self.skip_whitespace();

        if self.ch.is_alphabetic() {
            let literal: String = self.read_identifier();
            tok = Token {
                token_type: token::lookup_ident(&literal),
                literal: literal,
            }
        } else if self.ch.is_numeric() {
            tok = Token {
                token_type: TokenType::INT,
                literal: self.read_number(),
            }
        } else {
            tok = match self.ch {
                '=' => Token {
                    token_type: TokenType::ASSIGN,
                    literal: self.ch.to_string(),
                },
                ';' => Token {
                    token_type: TokenType::SEMICOLON,
                    literal: self.ch.to_string(),
                },
                '(' => Token {
                    token_type: TokenType::LPAREN,
                    literal: self.ch.to_string(),
                },
                ')' => Token {
                    token_type: TokenType::RPAREN,
                    literal: self.ch.to_string(),
                },
                ',' => Token {
                    token_type: TokenType::COMMA,
                    literal: self.ch.to_string(),
                },
                '+' => Token {
                    token_type: TokenType::PLUS,
                    literal: self.ch.to_string(),
                },
                '{' => Token {
                    token_type: TokenType::LBRACE,
                    literal: self.ch.to_string(),
                },
                '}' => Token {
                    token_type: TokenType::RBRACE,
                    literal: self.ch.to_string(),
                },
                '\0' => Token {
                    token_type: TokenType::EOF,
                    literal: self.ch.to_string(),
                },
                _ => Token {
                    token_type: TokenType::ILLEGAL,
                    literal: self.ch.to_string(),
                },
            }
        };

        self.read_char(); // read next char to advance positions before we return
                          // the token at current read position

        return tok;
    }

    pub fn read_identifier(&mut self) -> String {
        let position: usize = self.position;
        while self.ch.is_alphabetic() {
            self.read_char();
        }
        return self.input[position..self.position].to_string();
    }

    pub fn read_number(&mut self) -> String {
        let position: usize = self.position;
        while self.ch.is_numeric() {
            self.read_char();
        }
        return self.input[position..self.position].to_string();
    }

    pub fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }
}
