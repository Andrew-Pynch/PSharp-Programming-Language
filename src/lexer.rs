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
        println!("Generating all tokens!\n\n");

        let mut tokens = Vec::new();

        println!("Initialized tokens: {:?}", tokens);

        while self.ch != '\0' {
            let tok = self.next_token();
            tokens.push(tok);
        }

        self.tokens = tokens;
    }

    pub fn next_token(&mut self) -> Token {
        println!("Next token!\n\n");

        let token: Token;

        if self.ch == '=' {
            token = Token::new(TokenType::Eq, self.ch);
        } else if self.ch == '+' {
            token = Token::new(TokenType::Plus, self.ch);
        } else if self.ch == '-' {
            token = Token::new(TokenType::Minus, self.ch);
        } else if self.ch == '!' {
            token = Token::new(TokenType::Bang, self.ch);
        } else if self.ch == '*' {
            token = Token::new(TokenType::Asterisk, self.ch);
        } else if self.ch == '/' {
            token = Token::new(TokenType::Slash, self.ch);
        } else if self.ch == '<' {
            token = Token::new(TokenType::Lt, self.ch);
        } else if self.ch == '>' {
            token = Token::new(TokenType::Gt, self.ch);
        } else if self.ch == '=' {
            token = Token::new(TokenType::Eq, self.ch);
        } else if self.ch == '!' {
            token = Token::new(TokenType::Bang, self.ch);
        } else if self.ch == ',' {
            token = Token::new(TokenType::Comma, self.ch);
        } else if self.ch == ';' {
            token = Token::new(TokenType::Semicolon, self.ch);
        } else if self.ch == '(' {
            token = Token::new(TokenType::Lparen, self.ch);
        } else if self.ch == ')' {
            token = Token::new(TokenType::Rparen, self.ch);
        } else if self.ch == '{' {
            token = Token::new(TokenType::Lbrace, self.ch);
        } else if self.ch == '}' {
            token = Token::new(TokenType::Rbrace, self.ch);
        } else {
            token = Token::new(TokenType::Uninitialized, self.ch);
        }

        println!("Token: {:?}", token);

        self.read_char();

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
