use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenType {
    Uninitialized,

    Illegal,
    Eof,
    // Identifiers + literals
    Ident,
    Int,
    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    // Value comparison
    Lt,
    Gt,
    Eq,
    NotEq,
    // Delimiters
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
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
            "UNINITIALIZED" => Ok(TokenType::Illegal),

            "ILLEGAL" => Ok(TokenType::Illegal),
            "EOF" => Ok(TokenType::Eof),
            // Identifiers + literals
            "IDENT" => Ok(TokenType::Ident),
            "INT" => Ok(TokenType::Int),
            // Operators
            "=" => Ok(TokenType::Assign),
            "+" => Ok(TokenType::Plus),
            "-" => Ok(TokenType::Minus),
            "!" => Ok(TokenType::Bang),
            "*" => Ok(TokenType::Asterisk),
            "/" => Ok(TokenType::Slash),
            // Value comparison
            "<" => Ok(TokenType::Lt),
            ">" => Ok(TokenType::Gt),
            "==" => Ok(TokenType::Eq),
            "!=" => Ok(TokenType::NotEq),
            // Delimiters
            "," => Ok(TokenType::Comma),
            ";" => Ok(TokenType::Semicolon),
            "(" => Ok(TokenType::Lparen),
            ")" => Ok(TokenType::Rparen),
            "{" => Ok(TokenType::Lbrace),
            "}" => Ok(TokenType::Rbrace),
            // Keywords
            "FUNCTION" => Ok(TokenType::Function),
            "LET" => Ok(TokenType::Let),
            "CONST" => Ok(TokenType::Const),
            "TRUE" => Ok(TokenType::True),
            "FALSE" => Ok(TokenType::False),
            "IF" => Ok(TokenType::If),
            "ELSE" => Ok(TokenType::Else),
            "RETURN" => Ok(TokenType::Return),
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
    // pub fn new() -> Token {
    //     Token {
    //         token_type: TokenType::Uninitialized,
    //         literal: String::new(),
    //     }
    // }

    pub fn new(_token_type: TokenType, _literal: String) -> Token {
        return Token {
            token_type: _token_type,
            literal: _literal,
        };
    }

    // pub fn set(_token_type: TokenType, _literal: char) ->
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
