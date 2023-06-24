use crate::token::{Token, TokenType};
use std::fmt;

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
impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum StatementType {
    Blank,
    LetStatement(Identifier, Expression),
    ReturnStatement(Expression),
    Expression(Expression),
}

impl fmt::Display for StatementType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StatementType::Blank => write!(f, "Blank"),
            StatementType::LetStatement(identifier, expression) => {
                write!(f, "LetStatement({}, {})", identifier, expression)
            }
            StatementType::ReturnStatement(expression) => {
                write!(f, "ReturnStatement({})", expression)
            }
            StatementType::Expression(expression) => {
                write!(f, "Expression({})", expression)
            }
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Statement {
    pub token: Token, // the token.LET token
    pub statement_type: StatementType,
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.statement_type)
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Expression {
    Identifier(Identifier),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Identifier(identifier) => write!(f, "Identifier({})", identifier),
        }
    }
}

pub struct Program {
    pub statements: Vec<Statement>,
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for statement in &self.statements {
            write!(f, "{}", statement)?;
        }
        Ok(())
    }
}

impl Program {
    pub fn new() -> Program {
        Program { statements: vec![] }
    }
}
