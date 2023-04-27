use crate::token::{Token, TokenType};

#[derive(PartialEq, Clone, Debug)]
pub struct Identifier {
    pub token: Token, // the token.IDENT token
    pub value: String,
}

impl Identifier {
    pub fn new(tok: Token, val: String) -> Identifier {
        Identifier {
            token: tok,
            value: val,
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum StatementType {
    Blank,
    LetStatement(Identifier, Expression),
    ReturnStatement(Expression),
    Expression(Expression),
}

#[derive(PartialEq, Clone, Debug)]
pub struct Statement {
    pub token: Token, // the token.LET token
    pub statement_type: StatementType,
    pub name: Identifier,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Expression {
    Identifier(Identifier),
}

pub struct LetStatement {
    pub token: Token, // the token.LET token
    pub name: Identifier,
    pub value: Expression,
}

impl LetStatement {
    pub fn new(tok: Token) -> LetStatement {
        LetStatement {
            token: tok,
            name: Identifier::new(Token::new(TokenType::IDENT, "".to_string()), "".to_string()),
            value: Expression::Identifier(Identifier::new(
                Token::new(TokenType::IDENT, "".to_string()),
                "".to_string(),
            )),
        }
    }
}

pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Program {
        Program { statements: vec![] }
    }
}
