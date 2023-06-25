use psharp_programming_language::{
    ast::{Identifier, LetStatement, Program},
    token::{Token, TokenType},
};

#[test]
fn test_string() {
    let program = Program {
        statements: vec![Box::new(LetStatement {
            token: Token {
                token_type: TokenType::LET,
                literal: "let".to_string(),
            },
            name: Identifier {
                token: Token {
                    token_type: TokenType::IDENT,
                    literal: "myVar".to_string(),
                },
                value: "myVar".to_string(),
            },
            value: Some(Box::new(Identifier {
                token: Token {
                    token_type: TokenType::IDENT,
                    literal: "anotherVar".to_string(),
                },
                value: "anotherVar".to_string(),
            })),
        })],
    };

    assert_eq!(program.to_string(), "let myVar = anotherVar;");
}
