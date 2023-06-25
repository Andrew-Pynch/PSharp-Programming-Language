use crate::{
    ast::{self, Expression, Identifier, LetStatement, Program, ReturnStatement, Statement},
    lexer::Lexer,
    token::{Token, TokenType},
};

pub struct Parser {
    pub l: Lexer,

    pub cur_token: Token,
    pub peek_token: Token,
    pub errors: Vec<String>,
}

impl Parser {
    pub fn new(l: Lexer) -> Parser {
        let mut p = Parser {
            l: l.clone(),

            cur_token: Token::new(TokenType::UNINITIALIZED, "".to_string()),
            peek_token: Token::new(TokenType::UNINITIALIZED, "".to_string()),
            errors: Vec::new(),
        };

        p.next_token();
        p.next_token();

        return p;
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program: Program = Program {
            statements: Vec::new(),
        };

        while !self.cur_token_is(TokenType::EOF) {
            let mut stmt: Option<Box<Vec<dyn Statement>>> = self.parse_statement();
            if let Some(s) = stmt {
                program.statements.append(&mut *s);
            }
            self.next_token();
        }
        return program;
    }

    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    pub fn cur_token_is(&mut self, token_type: TokenType) -> bool {
        return self.cur_token.token_type == token_type;
    }

    pub fn peek_token_is(&mut self, token_type: TokenType) -> bool {
        return self.peek_token.token_type == token_type;
    }

    pub fn peek_error(&mut self, token_type: TokenType) {
        let msg = format!(
            "expected next token to be {:?}, got {:?} instead",
            token_type, self.peek_token.token_type
        );
        self.errors.push(msg);
    }

    pub fn expect_peek(&mut self, token_type: TokenType) -> bool {
        if self.peek_token_is(token_type) {
            self.next_token();
            return true;
        } else {
            self.peek_error(token_type);
            return false;
        }
    }

    // ...existing functions...

    pub fn parse_let_statement(&mut self) -> Option<Box<dyn Statement>> {
        let token = self.cur_token.clone();

        if !self.expect_peek(TokenType::IDENT) {
            return None;
        }

        let name = Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        };

        if !self.expect_peek(TokenType::ASSIGN) {
            return None;
        }

        // self.next_token();

        // statement.value = self.parse_expression(Precedence::LOWEST);

        if self.cur_token_is(TokenType::SEMICOLON) {
            self.next_token();
        };

        Some(Box::new(LetStatement {
            token,
            name,        // no need to box here
            value: None, // should be `Option<Box<dyn Expression>>`
        }))
    }

    pub fn parse_return_statement(&mut self) -> Option<Box<dyn Statement>> {
        let token = self.cur_token.clone();

        self.next_token();

        // statement.return_value = self.parse_expression(Precedence::LOWEST);

        if self.cur_token_is(TokenType::SEMICOLON) {
            self.next_token();
        };

        Some(Box::new(ReturnStatement {
            token,
            return_value: None, // should be `Option<Box<dyn Expression>>`
        }))
    }
}
