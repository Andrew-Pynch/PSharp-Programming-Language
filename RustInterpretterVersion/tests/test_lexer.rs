use psharp_programming_language::{
    lexer::Lexer,
    token::{Token, TokenType},
};

// let string = "line one
// line two";
fn get_test_input() -> &'static str {
    // let test_input: &str = "1{; -\0";
    let test_input: &str = "let five = 5;
        let ten = 10;
    
        let add = func(x, y) {
            x + y;
        };
        
        let result = add(five, ten);
        
        !-/*5;
        
        if (5 < 10) {
            return true;
        } else {
            return false;
        }
        
        10 == 10;
        10 != 9;
    ";
    return test_input;
}

#[test]
fn test_new() {
    let test_input = get_test_input();
    let lexer = Lexer::new(test_input);
    let test_lexer = Lexer {
        input: test_input.to_string(),
        position: 0,
        ch: test_input.as_bytes()[0] as char,
        tokens: Vec::new(),
        is_at_end: false,
    };
    assert_eq!(lexer, test_lexer);
}

#[test]
fn test_generate_all_tokens() {
    let test_input: &str = get_test_input();
    let mut lexer: Lexer = Lexer::new(test_input);
    lexer.generate_all_tokens();

    let test_tokens: Vec<Token> = vec![
        Token {
            token_type: TokenType::Let,
            literal: "let".to_string(),
        },
        Token {
            token_type: TokenType::Ident,
            literal: "five".to_string(),
        },
        Token {
            token_type: TokenType::Assign,
            literal: "=".to_string(),
        },
        Token {
            token_type: TokenType::Int,
            literal: "5".to_string(),
        },
        Token {
            token_type: TokenType::Semicolon,
            literal: ";".to_string(),
        },
        Token {
            token_type: TokenType::Let,
            literal: "let".to_string(),
        },
        Token {
            token_type: TokenType::Ident,
            literal: "ten".to_string(),
        },
        Token {
            token_type: TokenType::Assign,
            literal: "=".to_string(),
        },
        Token {
            token_type: TokenType::Int,
            literal: "10".to_string(),
        },
        Token {
            token_type: TokenType::Semicolon,
            literal: ";".to_string(),
        },
        Token {
            token_type: TokenType::Let,
            literal: "let".to_string(),
        },
        Token {
            token_type: TokenType::Ident,
            literal: "add".to_string(),
        },
        Token {
            token_type: TokenType::Assign,
            literal: "=".to_string(),
        },
        Token {
            token_type: TokenType::Function,
            literal: "func".to_string(),
        },
        Token {
            token_type: TokenType::LParen,
            literal: "(".to_string(),
        },
        Token {
            token_type: TokenType::Ident,
            literal: "x".to_string(),
        },
        Token {
            token_type: TokenType::Comma,
            literal: ",".to_string(),
        },
        Token {
            token_type: TokenType::Ident,
            literal: "y".to_string(),
        },
        Token {
            token_type: TokenType::RParen,
            literal: ")".to_string(),
        },
        Token {
            token_type: TokenType::LBrace,
            literal: "{".to_string(),
        },
        Token {
            token_type: TokenType::Ident,
            literal: "x".to_string(),
        },
        Token {
            token_type: TokenType::Plus,
            literal: "+".to_string(),
        },
        Token {
            token_type: TokenType::Ident,
            literal: "y".to_string(),
        },
        Token {
            token_type: TokenType::Semicolon,
            literal: ";".to_string(),
        },
        Token {
            token_type: TokenType::RBrace,
            literal: "}".to_string(),
        },
        Token {
            token_type: TokenType::Semicolon,
            literal: ";".to_string(),
        },
        Token {
            token_type: TokenType::Let,
            literal: "let".to_string(),
        },
        Token {
            token_type: TokenType::Ident,
            literal: "result".to_string(),
        },
        Token {
            token_type: TokenType::Assign,
            literal: "=".to_string(),
        },
        Token {
            token_type: TokenType::Ident,
            literal: "add".to_string(),
        },
        Token {
            token_type: TokenType::LParen,
            literal: "(".to_string(),
        },
        Token {
            token_type: TokenType::Ident,
            literal: "five".to_string(),
        },
        Token {
            token_type: TokenType::Comma,
            literal: ",".to_string(),
        },
        Token {
            token_type: TokenType::Ident,
            literal: "ten".to_string(),
        },
        Token {
            token_type: TokenType::RParen,
            literal: ")".to_string(),
        },
        Token {
            token_type: TokenType::Semicolon,
            literal: ";".to_string(),
        },
        Token {
            token_type: TokenType::Bang,
            literal: "!".to_string(),
        },
        Token {
            token_type: TokenType::Minus,
            literal: "-".to_string(),
        },
        Token {
            token_type: TokenType::Slash,
            literal: "/".to_string(),
        },
        Token {
            token_type: TokenType::Asterisk,
            literal: "*".to_string(),
        },
        Token {
            token_type: TokenType::Int,
            literal: "5".to_string(),
        },
        Token {
            token_type: TokenType::Semicolon,
            literal: ";".to_string(),
        },
        Token {
            token_type: TokenType::If,
            literal: "if".to_string(),
        },
        Token {
            token_type: TokenType::LParen,
            literal: "(".to_string(),
        },
        Token {
            token_type: TokenType::Int,
            literal: "5".to_string(),
        },
        Token {
            token_type: TokenType::Lt,
            literal: "<".to_string(),
        },
        Token {
            token_type: TokenType::Int,
            literal: "10".to_string(),
        },
        Token {
            token_type: TokenType::RParen,
            literal: ")".to_string(),
        },
        Token {
            token_type: TokenType::LBrace,
            literal: "{".to_string(),
        },
        Token {
            token_type: TokenType::Return,
            literal: "return".to_string(),
        },
        Token {
            token_type: TokenType::True,
            literal: "true".to_string(),
        },
        Token {
            token_type: TokenType::Semicolon,
            literal: ";".to_string(),
        },
        Token {
            token_type: TokenType::RBrace,
            literal: "}".to_string(),
        },
        Token {
            token_type: TokenType::Else,
            literal: "else".to_string(),
        },
        Token {
            token_type: TokenType::LBrace,
            literal: "{".to_string(),
        },
        Token {
            token_type: TokenType::Return,
            literal: "return".to_string(),
        },
        Token {
            token_type: TokenType::False,
            literal: "false".to_string(),
        },
        Token {
            token_type: TokenType::Semicolon,
            literal: ";".to_string(),
        },
        Token {
            token_type: TokenType::RBrace,
            literal: "}".to_string(),
        },
        Token {
            token_type: TokenType::Int,
            literal: "10".to_string(),
        },
        Token {
            token_type: TokenType::Eq,
            literal: "==".to_string(),
        },
        Token {
            token_type: TokenType::Int,
            literal: "10".to_string(),
        },
        Token {
            token_type: TokenType::Semicolon,
            literal: ";".to_string(),
        },
        Token {
            token_type: TokenType::Int,
            literal: "10".to_string(),
        },
        Token {
            token_type: TokenType::Neq,
            literal: "!=".to_string(),
        },
        Token {
            token_type: TokenType::Int,
            literal: "9".to_string(),
        },
        Token {
            token_type: TokenType::Semicolon,
            literal: ";".to_string(),
        },
        Token {
            token_type: TokenType::Eof,
            literal: "\0".to_string(),
        },
    ];
    assert_eq!(lexer.tokens.len(), test_tokens.len());

    for (i, token) in lexer.tokens.iter().enumerate() {
        dbg!(token, test_tokens[i].clone());
        assert_eq!(token, &test_tokens[i]);
    }

    assert_eq!(lexer.tokens, test_tokens);
}
