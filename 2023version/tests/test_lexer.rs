use psharp_programming_language::{
    lexer::Lexer,
    token::{Token, TokenType},
};

fn get_test_input() -> String {
    let test_input: String = "let five = 5; 
        let ten = 10;

        let add = func(x, y) {
            x + y;
        };

        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;

        if (5 < 10) {
            return true;
        } else {
            return false;
        }
        
        10 == 10;
        10 != 9;
    "
    .to_string();

    return test_input;
}

#[test]
fn test_new() {
    let test_input: String = get_test_input();
    let lexer: Lexer = Lexer::new(&test_input);
    let test_lexer: Lexer = Lexer {
        input: test_input.to_string(),
        position: 0,
        read_position: 0,
        ch: test_input.as_bytes()[0] as char,
    };
    assert_eq!(lexer, test_lexer);
}

#[test]
fn test_generate_all_tokens() {
    let test_input: String = get_test_input();

    let mut lexer: Lexer = Lexer::new(&test_input);

    let test_tokens: Vec<Token> = vec![
        Token {
            token_type: TokenType::LET,
            literal: "let".to_string(),
        },
        Token {
            token_type: TokenType::IDENT,
            literal: "five".to_string(),
        },
        Token {
            token_type: TokenType::ASSIGN,
            literal: "=".to_string(),
        },
        Token {
            token_type: TokenType::INT,
            literal: "5".to_string(),
        },
        Token {
            token_type: TokenType::SEMICOLON,
            literal: ";".to_string(),
        },
        Token {
            token_type: TokenType::LET,
            literal: "let".to_string(),
        },
        Token {
            token_type: TokenType::IDENT,
            literal: "ten".to_string(),
        },
        Token {
            token_type: TokenType::ASSIGN,
            literal: "=".to_string(),
        },
        Token {
            token_type: TokenType::INT,
            literal: "10".to_string(),
        },
        Token {
            token_type: TokenType::SEMICOLON,
            literal: ";".to_string(),
        },
        Token {
            token_type: TokenType::LET,
            literal: "let".to_string(),
        },
        Token {
            token_type: TokenType::IDENT,
            literal: "add".to_string(),
        },
        Token {
            token_type: TokenType::ASSIGN,
            literal: "=".to_string(),
        },
        Token {
            token_type: TokenType::FUNCTION,
            literal: "func".to_string(),
        },
        Token {
            token_type: TokenType::LPAREN,
            literal: "(".to_string(),
        },
        Token {
            token_type: TokenType::IDENT,
            literal: "x".to_string(),
        },
        Token {
            token_type: TokenType::COMMA,
            literal: ",".to_string(),
        },
        Token {
            token_type: TokenType::IDENT,
            literal: "y".to_string(),
        },
        Token {
            token_type: TokenType::RPAREN,
            literal: ")".to_string(),
        },
        Token {
            token_type: TokenType::LBRACE,
            literal: "{".to_string(),
        },
        Token {
            token_type: TokenType::IDENT,
            literal: "x".to_string(),
        },
        Token {
            token_type: TokenType::PLUS,
            literal: "+".to_string(),
        },
        Token {
            token_type: TokenType::IDENT,
            literal: "y".to_string(),
        },
        Token {
            token_type: TokenType::SEMICOLON,
            literal: ";".to_string(),
        },
        Token {
            token_type: TokenType::RBRACE,
            literal: "}".to_string(),
        },
        Token {
            token_type: TokenType::SEMICOLON,
            literal: ";".to_string(),
        },
        Token {
            token_type: TokenType::LET,
            literal: "let".to_string(),
        },
        Token {
            token_type: TokenType::IDENT,
            literal: "result".to_string(),
        },
        Token {
            token_type: TokenType::ASSIGN,
            literal: "=".to_string(),
        },
        Token {
            token_type: TokenType::IDENT,
            literal: "add".to_string(),
        },
        Token {
            token_type: TokenType::LPAREN,
            literal: "(".to_string(),
        },
        Token {
            token_type: TokenType::IDENT,
            literal: "five".to_string(),
        },
        Token {
            token_type: TokenType::COMMA,
            literal: ",".to_string(),
        },
        Token {
            token_type: TokenType::IDENT,
            literal: "ten".to_string(),
        },
        Token {
            token_type: TokenType::RPAREN,
            literal: ")".to_string(),
        },
        Token {
            token_type: TokenType::SEMICOLON,
            literal: ";".to_string(),
        },
        Token {
            token_type: TokenType::BANG,
            literal: "!".to_string(),
        },
        Token {
            token_type: TokenType::MINUS,
            literal: "-".to_string(),
        },
        Token {
            token_type: TokenType::SLASH,
            literal: "/".to_string(),
        },
        Token {
            token_type: TokenType::ASTERISK,
            literal: "*".to_string(),
        },
        Token {
            token_type: TokenType::INT,
            literal: "5".to_string(),
        },
        Token {
            token_type: TokenType::SEMICOLON,
            literal: ";".to_string(),
        },
        Token {
            token_type: TokenType::INT,
            literal: "5".to_string(),
        },
        Token {
            token_type: TokenType::LT,
            literal: "<".to_string(),
        },
        Token {
            token_type: TokenType::INT,
            literal: "10".to_string(),
        },
        Token {
            token_type: TokenType::GT,
            literal: ">".to_string(),
        },
        Token {
            token_type: TokenType::INT,
            literal: "5".to_string(),
        },
        Token {
            token_type: TokenType::SEMICOLON,
            literal: ";".to_string(),
        },
        Token {
            token_type: TokenType::IF,
            literal: "if".to_string(),
        },
        Token {
            token_type: TokenType::LPAREN,
            literal: "(".to_string(),
        },
        Token {
            token_type: TokenType::INT,
            literal: "5".to_string(),
        },
        Token {
            token_type: TokenType::LT,
            literal: "<".to_string(),
        },
        Token {
            token_type: TokenType::INT,
            literal: "10".to_string(),
        },
        Token {
            token_type: TokenType::RPAREN,
            literal: ")".to_string(),
        },
        Token {
            token_type: TokenType::LBRACE,
            literal: "{".to_string(),
        },
        Token {
            token_type: TokenType::RETURN,
            literal: "return".to_string(),
        },
        Token {
            token_type: TokenType::TRUE,
            literal: "true".to_string(),
        },
        Token {
            token_type: TokenType::SEMICOLON,
            literal: ";".to_string(),
        },
        Token {
            token_type: TokenType::RBRACE,
            literal: "}".to_string(),
        },
        Token {
            token_type: TokenType::ELSE,
            literal: "else".to_string(),
        },
        Token {
            token_type: TokenType::LBRACE,
            literal: "{".to_string(),
        },
        Token {
            token_type: TokenType::RETURN,
            literal: "return".to_string(),
        },
        Token {
            token_type: TokenType::FALSE,
            literal: "false".to_string(),
        },
        Token {
            token_type: TokenType::SEMICOLON,
            literal: ";".to_string(),
        },
        Token {
            token_type: TokenType::RBRACE,
            literal: "}".to_string(),
        },
        Token {
            token_type: TokenType::INT,
            literal: "10".to_string(),
        },
        Token {
            token_type: TokenType::EQ,
            literal: "==".to_string(),
        },
        Token {
            token_type: TokenType::INT,
            literal: "10".to_string(),
        },
        Token {
            token_type: TokenType::SEMICOLON,
            literal: ";".to_string(),
        },
        Token {
            token_type: TokenType::INT,
            literal: "10".to_string(),
        },
        Token {
            token_type: TokenType::NOT_EQ,
            literal: "!=".to_string(),
        },
        Token {
            token_type: TokenType::INT,
            literal: "9".to_string(),
        },
        Token {
            token_type: TokenType::SEMICOLON,
            literal: ";".to_string(),
        },
    ];

    // iterate through all the test tokens and make sure that the lexers token
    // and the test token == each other at each index

    for test_token in test_tokens.iter() {
        let token: Token = lexer.next_token();

        println!("\n  REAL: {:?}\n  TEST: {:?}", token, test_token);

        // Assert that the token and test_token match
        assert_eq!(&token, test_token);

        // Break out of the loop if the current token type is EOF
        if token.token_type == TokenType::EOF {
            break;
        }
    }
}
