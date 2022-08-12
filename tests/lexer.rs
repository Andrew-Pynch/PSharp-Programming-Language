use psharp_programming_language::{
    lexer::Lexer,
    token::{Token, TokenType},
};

// let string = "line one
// line two";
fn get_test_input() -> &'static str {
    let test_input: &str = "1 a {";
    return test_input;
}

#[test]
fn test_new() {
    let test_input = get_test_input();
    let lexer = Lexer::new(test_input);
    let test_lexer = Lexer {
        input: test_input.to_string(),
        position: 0,
        read_position: 0,
        ch: test_input.as_bytes()[0] as char,
        tokens: Vec::new(),
    };
    assert_eq!(lexer, test_lexer);
}

#[test]
fn test_generate_all_tokens() {
    let test_input = get_test_input();
    let mut lexer = Lexer::new(test_input);
    lexer.generate_all_tokens();
    let test_tokens = vec![
        Token {
            token_type: TokenType::Int,
            literal: "1".to_string(),
        },
        Token {
            token_type: TokenType::Ident,
            literal: "a".to_string(),
        },
        Token {
            token_type: TokenType::Lbrace,
            literal: "{".to_string(),
        },
    ];
    assert_eq!(lexer.tokens, test_tokens);
}
