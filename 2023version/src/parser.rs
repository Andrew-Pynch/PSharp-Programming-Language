use crate::{
    ast::{self, Expression, Identifier, Program, Statement, StatementType},
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
    pub fn new(_l: &Lexer) -> Parser {
        let mut p: Parser = Parser {
            l: _l.clone(),

            cur_token: Token::new(TokenType::UNINITIALIZED, "".to_string()),
            peek_token: Token::new(TokenType::UNINITIALIZED, "".to_string()),
            errors: Vec::new(),
        };
        return p;
    }

    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program: Program = Program::new();

        while self.cur_token.token_type != TokenType::EOF {
            match self.parse_statement() {
                Some(stmt) => program.statements.push(stmt),
                None => {}
            }
            self.next_token()
        }

        return program;
    }

    pub fn parse_statement(&mut self) -> Option<Statement> {
        match self.cur_token.token_type {
            TokenType::LET => self.parse_let_statement(),
            TokenType::RETURN => self.parse_return_statement(),
            _ => None,
        }
    }

    pub fn parse_let_statement(&mut self) -> Option<Statement> {
        let token = self.cur_token.clone();

        if !self.expect_peek(TokenType::IDENT) {
            return None;
        }

        let name = Identifier::new(self.cur_token.clone(), self.cur_token.literal.clone());

        if !self.expect_peek(TokenType::ASSIGN) {
            return None;
        }

        // self.next_token();

        // statement.value = self.parse_expression(Precedence::LOWEST);

        if self.current_token_is(TokenType::SEMICOLON) {
            self.next_token();
        };

        return Some(Statement {
            token,
            statement_type: StatementType::LetStatement(
                name,
                Expression::Identifier(Identifier::new(
                    Token::new(TokenType::IDENT, "".to_string()),
                    "".to_string(),
                )),
            ),
        });
    }

    pub fn parse_return_statement(&mut self) -> Option<Statement> {
        let token = self.cur_token.clone();

        self.next_token();

        // statement.value = self.parse_expression(Precedence::LOWEST);

        if self.current_token_is(TokenType::SEMICOLON) {
            self.next_token();
        };

        return Some(Statement {
            token,
            statement_type: StatementType::ReturnStatement(Expression::Identifier(
                Identifier::new(Token::new(TokenType::IDENT, "".to_string()), "".to_string()),
            )),
        });
    }

    pub fn current_token_is(&mut self, tok: TokenType) -> bool {
        return self.cur_token.token_type == tok;
    }

    pub fn peek_token_is(&mut self, tok: TokenType) -> bool {
        return self.peek_token.token_type == tok;
    }

    pub fn expect_peek(&mut self, tok: TokenType) -> bool {
        if self.peek_token_is(tok) {
            self.next_token();
            return true;
        } else {
            self.peek_error(tok);
            return false;
        }
    }

    pub fn peek_error(&mut self, tok: TokenType) {
        let msg = format!(
            "expected next token to be {:?}, got {:?} instead",
            tok, self.peek_token.token_type
        );
        self.errors.push(msg);
    }
}
