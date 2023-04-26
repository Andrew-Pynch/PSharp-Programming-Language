use std::str::FromStr;

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

pub const KEYWORDS: [&str; 8] = [
    "func", "let", "const", "true", "false", "if", "else", "return",
];

pub fn lookup_ident(ident: &str) -> TokenType {
    // Check if the identifier is a keyword
    if KEYWORDS.contains(&ident) {
        // If the identifier is a keyword, use the FromStr trait to convert it to the corresponding TokenType
        match TokenType::from_str(ident) {
            Ok(token_type) => token_type,
            Err(_) => TokenType::ILLEGAL, // This case should not occur if the keyword is valid
        }
    } else {
        // If the identifier is not a keyword, return TokenType::IDENT
        TokenType::IDENT
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    // Identifiers + literals
    IDENT,
    INT,

    // Operators
    ASSIGN,
    PLUS,

    // Delimiters
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // Keywords
    FUNCTION,
    LET,
}

#[derive(Debug, PartialEq)]
pub struct NotAToken;

impl FromStr for TokenType {
    type Err = NotAToken;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ILLEGAL" => Ok(TokenType::ILLEGAL),
            "EOF" => Ok(TokenType::EOF),

            // Identifiers
            "IDENT" => Ok(TokenType::IDENT),
            "INT" => Ok(TokenType::INT),

            // Operators
            "=" => Ok(TokenType::ASSIGN),
            "+" => Ok(TokenType::PLUS),

            // Delimiters
            "," => Ok(TokenType::COMMA),
            ";" => Ok(TokenType::SEMICOLON),

            "(" => Ok(TokenType::LPAREN),
            ")" => Ok(TokenType::RPAREN),
            "{" => Ok(TokenType::LBRACE),
            "}" => Ok(TokenType::RBRACE),

            // Keywords
            "func" => Ok(TokenType::FUNCTION),
            "let" => Ok(TokenType::LET),
            _ => Err(NotAToken),
        }
    }
}
