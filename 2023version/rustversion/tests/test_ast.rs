use psharp_programming_language::{
    ast::{Expression, Identifier, Program, Statement, StatementType},
    token::{Token, TokenType},
};

#[test]
fn test_string() {
    let program = Program {
        statements: vec![Statement {
            token: Token {
                token_type: TokenType::LET,
                literal: "let".to_string(),
            },
            statement_type: StatementType::LetStatement(
                Identifier {
                    token: Token {
                        token_type: TokenType::IDENT,
                        literal: "myVar".to_string(),
                    },
                    value: "myVar".to_string(),
                },
                Expression::Identifier(Identifier {
                    token: Token {
                        token_type: TokenType::IDENT,
                        literal: "anotherVar".to_string(),
                    },
                    value: "anotherVar".to_string(),
                }),
            ),
        }],
    };

    assert_eq!(format!("{}", program), "let myVar = anotherVar;");
}
