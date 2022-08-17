use crate::token::{Token, TokenType};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Lexer {
    pub input: String,
    pub position: usize, // current posistion in input (points to curr char)
    pub ch: char,        // current char under examination
    pub tokens: Vec<Token>,
    pub is_at_end: bool, // if we have reached the end of the stream
}

impl Lexer {
    pub fn new(_input: &str) -> Lexer {
        let l = Lexer {
            input: _input.to_string(),
            position: 0,
            ch: _input.as_bytes()[0] as char,
            tokens: Vec::new(),
            is_at_end: false,
        };

        // TODO: Implement this
        // l.read_char()

        return l;
    }

    pub fn increment_position_set_char(&mut self) {
        if self.position >= self.input.len() - 1 {
            self.ch = '\0';
        } else {
            self.position += 1;
            self.ch = self.input.as_bytes()[self.position] as char;
        }
    }

    pub fn generate_all_tokens(&mut self) {
        println!("Generating lexer tokens...");
        let mut _tokens = Vec::new();

        while !self.is_at_end {
            _tokens.push(self.get_token_at_current_block());
        }

        self.tokens = _tokens;
    }

    pub fn char_is_value_comparator(&mut self, ch: char) -> bool {
        let conditions: [bool; 4] = [ch == '=', ch == '!', ch == '<', ch == '>'];

        for condition in conditions.iter() {
            if *condition {
                return true;
            }
        }

        return false;
    }

    pub fn char_is_operand(&mut self, ch: char) -> bool {
        let conditions: [bool; 4] = [ch == '+', ch == '-', ch == '*', ch == '/'];

        for condition in conditions.iter() {
            if *condition {
                return true;
            }
        }

        return false;
    }

    pub fn get_operand_or_value_comparator_token(&mut self) -> Token {
        let mut token: Token = Token::new(TokenType::Uninitialized, String::new());
        let mut next_char: char;

        // if we have another char above us
        if self.position + 1 < self.input.len() - 1 {
            next_char = self.input.as_bytes()[self.position + 1] as char;
        } else {
            next_char = '\0';
        }

        // if the next char is a value comparator
        // i.e dont want to match if char == '!' and next char is whitespace or
        // something else
        if self.char_is_value_comparator(next_char) || self.char_is_operand(next_char) {
            if self.ch == '+' && next_char == '=' {
                token = Token::new(TokenType::PlusAssign, "+=".to_string());
            } else if self.ch == '-' && next_char == '=' {
                token = Token::new(TokenType::MinusAssign, "-=".to_string());
            } else if self.ch == '*' && next_char == '=' {
                token = Token::new(TokenType::AsteriskAssign, "*=".to_string());
            } else if self.ch == '/' && next_char == '=' {
                token = Token::new(TokenType::SlashAssign, "/=".to_string());
            } else if self.ch == '<' && next_char == '=' {
                token = Token::new(TokenType::Lte, "<=".to_string());
            } else if self.ch == '>' && next_char == '=' {
                token = Token::new(TokenType::Gte, ">=".to_string());
            } else if self.ch == '=' && next_char == '=' {
                token = Token::new(TokenType::Eq, "==".to_string());
            } else if self.ch == '!' && next_char == '=' {
                token = Token::new(TokenType::Neq, "!=".to_string());
            }
            self.increment_position_set_char();
        }
        // otherwise, just return the token of the current char
        else {
            if self.ch == '=' {
                token = Token::new(TokenType::Assign, "=".to_string());
            } else if self.ch == '+' {
                token = Token::new(TokenType::Plus, "+".to_string());
            } else if self.ch == '-' {
                token = Token::new(TokenType::Minus, "-".to_string());
            } else if self.ch == '!' {
                token = Token::new(TokenType::Bang, "!".to_string());
            } else if self.ch == '/' {
                token = Token::new(TokenType::Slash, "/".to_string());
            } else if self.ch == '*' {
                token = Token::new(TokenType::Asterisk, "*".to_string());
            } else if self.ch == '<' {
                token = Token::new(TokenType::Lt, "<".to_string());
            } else if self.ch == '>' {
                token = Token::new(TokenType::Gt, ">".to_string());
            }
        }

        return token;
    }

    pub fn get_token_at_current_block(&mut self) -> Token {
        let mut token: Token = Token::new(TokenType::Uninitialized, "".to_string());

        self.skip_whitespace();

        // Value comparison >=, >, <=, <, ==, !=
        if self.char_is_value_comparator(self.ch) || self.char_is_operand(self.ch) {
            // COLLAPSE THIS INTO ONE FUNCTION THAT GETS OPERANDS AND VALUE COMPARISON
            token = self.get_operand_or_value_comparator_token();
        }
        // Delimiters
        else if self.ch == ',' {
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
        } else if self.ch == '[' {
            token = Token::new(TokenType::Lbrace, self.ch.to_string());
        } else if self.ch == ']' {
            token = Token::new(TokenType::Rbrace, self.ch.to_string());
        } else if self.ch == '\0' {
            token = Token::new(TokenType::Eof, self.ch.to_string());
            self.is_at_end = true;
        } else {
            if self.ch.is_alphabetic() {
                let identifier_chars = self.read_identifier();
                token = Token::new(TokenType::Ident, identifier_chars.to_string());
            } else if self.ch.is_numeric() {
                let number_chars = self.read_number();
                token = Token::new(TokenType::Int, number_chars.to_string());
            } else {
                // this line might be a problem
                token = Token::new(TokenType::Illegal, self.ch.to_string());
            }
        }

        self.increment_position_set_char();

        return token;
    }

    pub fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.increment_position_set_char();
        }
    }

    pub fn is_whitespace(&mut self, ch: char) -> bool {
        return ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r';
    }

    pub fn read_identifier(&mut self) -> String {
        let mut identifier_chars: Vec<char> = Vec::new();

        while self.ch.is_alphabetic() {
            identifier_chars.push(self.ch);
            self.increment_position_set_char();
        }

        return identifier_chars.into_iter().collect();
    }

    pub fn read_number(&mut self) -> String {
        let mut number_chars: Vec<char> = Vec::new();

        while self.ch.is_numeric() {
            number_chars.push(self.ch);
            self.increment_position_set_char();
        }

        return number_chars.into_iter().collect();
    }
}
