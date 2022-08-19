use std::io::{self, Write};

use crate::lexer::Lexer;

pub const PROMPT: &str = ">> ";

pub fn start() {
    println!("Welcome the P# programming language!");
    println!("Type exit to exit. \n\n");

    let mut counter: usize = 0;

    // print the prompt then read from the standard input
    'outer_counter: loop {
        if counter == 1 {
            break 'outer_counter;
        }
        counter += 1;

        print!("{}", PROMPT);
        // io::stdout().flush().unwrap();
        // let mut input = String::new();
        // io::stdin().read_line(&mut input).unwrap();
        let input = "!-/*5;";

        if input.len() == 0 {
            continue;
        }
        if input == "exit" {
            break;
        }

        let mut lexer: Lexer = Lexer::new(&input);
        lexer.generate_all_tokens();

        println!("Lexer Tokens: \n {:?}", lexer.tokens);
    }
}
