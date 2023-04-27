use std::io::{self, Write};

use crate::{
    lexer::Lexer,
    token::{Token, TokenType},
};

pub const PROMPT: &str = ">> ";

const PSHARP: &str = "
 mmmmm          mmmm  #                          
 #   \"#        #\"   \" # mm    mmm    m mm  mmmm  
 #mmm#\"        \"#mmm  #\"  #  \"   #   #\"  \" #\"# 
 #                 \"# #   #  m\"\"\"#   #     #   # 
 #             \"mmm#\" #   #  \"mm\"#   #     ##m#\" 
                                           #                                    \"  
";
pub fn start() {
    println!("{}\n\n", PSHARP);

    println!("Welcome the P# programming language!");
    println!("Type exit to exit. \n\n");

    loop {
        print!("\n\n{}", PROMPT);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "exit" {
            break;
        }

        // combine input with "let;"
        // input.push_str("let;");
        let mut lexer = Lexer::new(&input);
        let mut tokens = Vec::new();

        loop {
            let tok = lexer.next_token();
            tokens.push(tok.clone());
            println!("\n Token: {:?} \nLexer: {:?}", tok, lexer);

            if tok.token_type == TokenType::EOF {
                break;
            }
        }

        // println!("Lexer Tokens: \n {:?}", tokens);
    }
}
