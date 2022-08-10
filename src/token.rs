#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    TtInt,
    TtFloat,
    TtString,
    TtIdentifier,
    TtKeyword,
    TtPlus,
    TtMinus,
    TtMul,
    TtDiv,
    TtPow,
    TtEq,
    TtLparen,
    TtParenN,
    TtLsquare,
    TtRsquare,
    TtEe,
    TtNe,
    TtLt,
    TtGt,
    TtLte,
    TtGte,
    TtComma,
    TtSemicolon,
    TtArrow,
    TtNewline,
    TtEof,
}

static KEYWORDS: [&str; 11] = [
    "if", "else", "elif", "while", "and", "or", "not", "func", "return", "continue", "break",
];

impl TokenType {
    pub fn as_str(&self) -> &'static str {
        match self {
            TokenType::TtInt => "TtInt",
            TokenType::TtFloat => "TtFloat",
            TokenType::TtString => "TtString",
            TokenType::TtIdentifier => "TtIdentifier",
            TokenType::TtKeyword => "TtKeyword",
            TokenType::TtPlus => "TtPlus",
            TokenType::TtMinus => "TtMinus",
            TokenType::TtMul => "TtMul",
            TokenType::TtDiv => "TtDiv",
            TokenType::TtPow => "TtPow",
            TokenType::TtEq => "TtEq",
            TokenType::TtLparen => "TtLparen",
            TokenType::TtParenN => "TtParenN",
            TokenType::TtLsquare => "TtLsquare",
            TokenType::TtRsquare => "TtRsquare",
            TokenType::TtEe => "TtEe",
            TokenType::TtNe => "TtNe",
            TokenType::TtLt => "TtLt",
            TokenType::TtGt => "TtGt",
            TokenType::TtLte => "TtLte",
            TokenType::TtGte => "TtGte",
            TokenType::TtComma => "TtComma",
            TokenType::TtSemicolon => "TtSemicolon",
            TokenType::TtArrow => "TtArrow",
            TokenType::TtNewline => "TtNewline",
            TokenType::TtEof => "TtEof",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub token_value: String,
}
