use std::fmt::{self, Debug};

use crate::token;

// The base Node trait
pub trait Node {
    fn token_literal(&self) -> String;
    fn to_string(&self) -> String;
}

// All statement nodes implement this
pub trait Statement: Node {}
impl Debug for dyn Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Statement")
    }
}

// All expression nodes implement this
pub trait Expression: Node {}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}
impl ToString for Program {
    fn to_string(&self) -> String {
        self.statements
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join("")
    }
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if !self.statements.is_empty() {
            self.statements[0].token_literal()
        } else {
            "".to_string()
        }
    }

    fn to_string(&self) -> String {
        self.statements
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join("")
    }
}

// Statements
pub struct LetStatement {
    pub token: token::Token, // the token::LET token
    pub name: Identifier,
    pub value: Option<Box<dyn Expression>>,
}

impl Statement for LetStatement {}
impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn to_string(&self) -> String {
        format!(
            "{} {} = {};",
            self.token_literal(),
            self.name.to_string(),
            self.value
                .as_ref()
                .map(|v| v.to_string())
                .unwrap_or_else(|| "".to_string())
        )
    }
}

pub struct ReturnStatement {
    pub token: token::Token, // the token::RETURN token
    pub return_value: Option<Box<dyn Expression>>,
}

impl Statement for ReturnStatement {}
impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn to_string(&self) -> String {
        format!(
            "{} {};",
            self.token_literal(),
            self.return_value
                .as_ref()
                .map(|v| v.to_string())
                .unwrap_or_else(|| "".to_string())
        )
    }
}

pub struct ExpressionStatement {
    pub token: token::Token, // the first token of the expression
    pub expression: Option<Box<dyn Expression>>,
}

impl Statement for ExpressionStatement {}
impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn to_string(&self) -> String {
        self.expression
            .as_ref()
            .map(|e| e.to_string())
            .unwrap_or_else(|| "".to_string())
    }
}

pub struct BlockStatement {
    pub token: token::Token, // the token::LBRACE token
    pub statements: Vec<Option<Box<dyn Statement>>>,
}

impl Statement for BlockStatement {}
impl Node for BlockStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn to_string(&self) -> String {
        self.statements
            .iter()
            .filter_map(|s| s.as_ref().map(|stmt| stmt.to_string()))
            .collect::<Vec<String>>()
            .join("")
    }
}

// Expressions
pub struct Identifier {
    pub token: token::Token, // the token::IDENT token
    pub value: String,
}

impl Expression for Identifier {}
impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn to_string(&self) -> String {
        self.value.clone()
    }
}

pub struct IntegerLiteral {
    pub token: token::Token, // the token::INT token
    pub value: i64,
}

impl Expression for IntegerLiteral {}
impl Node for IntegerLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn to_string(&self) -> String {
        self.token.literal.clone()
    }
}

pub struct Boolean {
    pub token: token::Token, // the token::TRUE or token::FALSE token
    pub value: bool,
}

impl Expression for Boolean {}
impl Node for Boolean {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn to_string(&self) -> String {
        self.token.literal.clone()
    }
}

pub struct PrefixExpression {
    pub token: token::Token, // The prefix token, e.g. token::BANG or token::MINUS
    pub operator: String,
    pub right: Box<dyn Expression>,
}

impl Expression for PrefixExpression {}
impl Node for PrefixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn to_string(&self) -> String {
        format!("({}{})", self.operator, self.right.to_string())
    }
}

pub struct InfixExpression {
    pub token: token::Token, // The operator token, e.g. token::PLUS, token::EQ, etc.
    pub left: Box<dyn Expression>,
    pub operator: String,
    pub right: Box<dyn Expression>,
}

impl Expression for InfixExpression {}
impl Node for InfixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn to_string(&self) -> String {
        format!(
            "({} {} {})",
            self.left.to_string(),
            self.operator,
            self.right.to_string()
        )
    }
}
