mod file_utils;

fn main() {
    // let rt = tokio::runtime::Runtime::new().unwrap();
    // let run_lexer_async =
    // lexer::lex_file()

    let user = std::env::var("USER").unwrap();

    print!("Hello {}! Welcome to the P# interpretter.", user);
    print!("You are currently in interactive mode. Type exit to exit");

    // start the repl with
    // repl.Start(std::io::stdin(), std::io::stdout())
}
