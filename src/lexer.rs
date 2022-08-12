use crate::file_utils;
use crate::position::Position;
use crate::token::{Token, TokenType};

pub struct Lexer {
    pub file_name: String,
    pub file_contents: String,
    pub position: Position,
    pub current_char: char,
}

pub fn lex_file() {
    let file_to_parse: String = "./src/psharp_programs/HelloWorld.pspl".to_string();
    let contents: String = file_utils::parse_pspl_file(file_to_parse.clone());

    // instantiate a new lexer struct
    let mut lexer: Lexer = Lexer {
        file_name: file_to_parse.clone(),
        file_contents: contents.clone(),
        position: Position {
            idx: 0,
            line: 1,
            col: 1,
            file_name: file_to_parse.clone(),
            file_contents: contents.clone(),
        },
        current_char: contents.clone().as_bytes()[0] as char,
    };

    println!("Current char {}", lexer.current_char);

    let tokens = get_tokens(lexer);
}

pub fn get_tokens(lexer: Lexer) {
    let mut tokens: Vec<Token> = Vec::new();

    tokens.push(get_next_token(lexer));

    println!("Tokens {:?}", tokens);
}

pub fn get_next_token(lexer: Lexer) -> Token {
    let mut token: Token = Token {
        token_type: TokenType::TtInt,
        token_value: "".to_string(),
    };
    return token;
}
