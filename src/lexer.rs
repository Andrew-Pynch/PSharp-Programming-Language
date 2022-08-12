use crate::token::{Token, TokenType};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Lexer {
    pub input: String,
    pub position: usize,      // current posistion in input (points to curr char)
    pub read_position: usize, // current reading position in input (after current char)
    pub ch: char,             // current char under examination
    pub tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(_input: &str) -> Lexer {
        let l = Lexer {
            input: _input.to_string(),
            position: 0,
            read_position: 0,
            ch: _input.as_bytes()[0] as char,
            tokens: Vec::new(),
        };

        // TODO: Implement this
        // l.read_char()

        return l;
    }

    pub fn generate_all_tokens(&mut self) {
        let mut tokens = Vec::new();

        loop {
            let tok: Token = self.next_token();

            if tok.token_type == TokenType::Eof {
                break;
            } else {
                tokens.push(tok);
                continue;
            }
        }

        self.tokens = tokens;
    }

    pub fn next_token(&mut self) -> Token {
        let token: Token;

        self.skip_whitespace();

        if self.ch == '=' {
            token = Token::new(TokenType::Eq, self.ch.to_string());
        } else if self.ch == '+' {
            token = Token::new(TokenType::Plus, self.ch.to_string());
        } else if self.ch == '-' {
            token = Token::new(TokenType::Minus, self.ch.to_string());
        } else if self.ch == '!' {
            token = Token::new(TokenType::Bang, self.ch.to_string());
        } else if self.ch == '*' {
            token = Token::new(TokenType::Asterisk, self.ch.to_string());
        } else if self.ch == '/' {
            token = Token::new(TokenType::Slash, self.ch.to_string());
        } else if self.ch == '<' {
            token = Token::new(TokenType::Lt, self.ch.to_string());
        } else if self.ch == '>' {
            token = Token::new(TokenType::Gt, self.ch.to_string());
        } else if self.ch == '=' {
            token = Token::new(TokenType::Eq, self.ch.to_string());
        } else if self.ch == '!' {
            token = Token::new(TokenType::Bang, self.ch.to_string());
        } else if self.ch == ',' {
            token = Token::new(TokenType::Comma, self.ch.to_string());
        } else if self.ch == ';' {
            token = Token::new(TokenType::Semicolon, self.ch.to_string());
        } else if self.ch == '(' {
            token = Token::new(TokenType::Lparen, self.ch.to_string());
        } else if self.ch == ')' {
            token = Token::new(TokenType::Rparen, self.ch.to_string());
        } else if self.ch == '{' {
            token = Token::new(TokenType::Lbrace, self.ch.to_string());
        } else if self.ch == '}' {
            token = Token::new(TokenType::Rbrace, self.ch.to_string());
        } else if self.ch == '\0' {
            token = Token::new(TokenType::Eof, self.ch.to_string());
        } else {
            if self.ch.is_alphabetic() {
                let literal: String = self.read_identifier();
                token = Token::new(TokenType::Ident, literal);
            } else if self.ch.is_numeric() {
                let literal: String = self.read_number();
                token = Token::new(TokenType::Int, literal);
            } else {
                token = Token::new(TokenType::Illegal, self.ch.to_string());
            }
        }

        self.read_char();

        return token;
    }

    pub fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }

    pub fn is_whitespace(&mut self, ch: char) -> bool {
        return ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r';
    }

    pub fn read_char(&mut self) {
        if self.position >= self.input.chars().count() {
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
        return self.input[position..self.position - 1].to_string();
    }

    pub fn read_number(&mut self) -> String {
        let position: usize = self.position;
        while self.ch.is_digit(10) {
            self.read_char();
        }
        return self.input[position..self.position - 1].to_string();
    }
}
