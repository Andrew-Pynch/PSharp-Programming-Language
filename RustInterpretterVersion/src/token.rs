use std::str::FromStr;

pub const KEYWORDS: [&str; 8] = [
    "func", "let", "const", "true", "false", "if", "else", "return",
];

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenType {
    Uninitialized,

    Illegal,
    Eof, // '\0'
    // Identifiers + literals
    Ident,
    Int,
    // ============
    // solo samuel's
    // ============
    // Operators
    Assign,   // =
    Plus,     // +
    Minus,    // -
    Bang,     // ! returns the inverse of an expression. !true is false, !false is true.
    Asterisk, // *
    Slash,    // /
    // Value comparison
    Lt, // <
    Gt, // >
    // ==============
    // double dussy's
    // ==============
    // Operators
    PlusAssign,
    MinusAssign,
    AsteriskAssign,
    SlashAssign,
    // Value comparison
    Lte,
    Gte,
    Eq,
    Neq,
    // Delimiters
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    // Keywords
    Function,
    Let,
    Const,
    True,
    False,
    If,
    Else,
    Return,
}
#[derive(Debug, PartialEq)]
pub struct NotAToken; // TODO: Improve this

impl FromStr for TokenType {
    type Err = NotAToken;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // "UNINITIALIZED" => Ok(TokenType::Illegal),

            // "illegal" => Ok(TokenType::Illegal),
            // "" => Ok(TokenType::Eof),
            // Identifiers + literals
            // "IDENT" => Ok(TokenType::Ident),
            // "INT" => Ok(TokenType::Int),
            // Operators
            "=" => Ok(TokenType::Assign),
            "+" => Ok(TokenType::Plus),
            "-" => Ok(TokenType::Minus),
            "!" => Ok(TokenType::Bang),
            "*" => Ok(TokenType::Asterisk),
            "/" => Ok(TokenType::Slash),
            "+=" => Ok(TokenType::PlusAssign),
            "-=" => Ok(TokenType::MinusAssign),
            "*=" => Ok(TokenType::AsteriskAssign),
            "/=" => Ok(TokenType::SlashAssign),
            // Value comparison
            "<" => Ok(TokenType::Lt),
            "<=" => Ok(TokenType::Lte),
            ">" => Ok(TokenType::Gt),
            ">=" => Ok(TokenType::Gte),
            "==" => Ok(TokenType::Eq),
            "!=" => Ok(TokenType::Neq),
            // Delimiters
            "," => Ok(TokenType::Comma),
            ";" => Ok(TokenType::Semicolon),
            "(" => Ok(TokenType::LParen),
            ")" => Ok(TokenType::RParen),
            "{" => Ok(TokenType::LBrace),
            "}" => Ok(TokenType::RBrace),
            // Keywords
            "func" => Ok(TokenType::Function),
            "let" => Ok(TokenType::Let),
            "const" => Ok(TokenType::Const),
            "true" => Ok(TokenType::True),
            "false" => Ok(TokenType::False),
            "if" => Ok(TokenType::If),
            "else" => Ok(TokenType::Else),
            "return" => Ok(TokenType::Return),
            _ => Err(NotAToken),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(_token_type: TokenType, _literal: String) -> Token {
        return Token {
            token_type: _token_type,
            literal: _literal,
        };
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "\nToken({:?}, {}\n)", self.token_type, self.literal)
    }
}

pub fn lookup_identifier(identifier: &str) -> TokenType {
    let token_type = TokenType::from_str(identifier).unwrap();
    return token_type;
}

// function to check if a string is a keyword
pub fn is_keyword(identifier_chars: String) -> bool {
    for keyword in KEYWORDS {
        if keyword == identifier_chars {
            return true;
        }
    }
    return false;
}

// function to return the keyword for a string literal
pub fn lookup_keyword_token_type(identifier_chars: String) -> TokenType {
    if identifier_chars == "func" {
        return TokenType::Function;
    } else if identifier_chars == "let" {
        return TokenType::Let;
    } else if identifier_chars == "const" {
        return TokenType::Const;
    } else if identifier_chars == "true" {
        return TokenType::True;
    } else if identifier_chars == "false" {
        return TokenType::False;
    } else if identifier_chars == "if" {
        return TokenType::If;
    } else if identifier_chars == "else" {
        return TokenType::Else;
    } else if identifier_chars == "return" {
        return TokenType::Return;
    } else {
        dbg!(identifier_chars.clone());
        return TokenType::Uninitialized;
    }
}
