use slang::lexer::Lexer;
use slang::token::TokenType;
use std::io::{stdin, stdout, Write};

const PROMPT: &str = ">> ";
const VERSION: &str = "0.0.1";

fn main() {
    println!("Slang {}", VERSION);

    loop {
        if print_prompt().is_err() {
            println!("Error printing prompt")
        }

        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(_) => match Lexer::new(Some(&input)) {
                Ok(lex) => print_tokens_from_line(lex),
                Err(error) => println!("{}", error),
            },
            Err(error) => println!("REPL error: {}", error),
        }
    }
}

fn print_tokens_from_line(lex: Lexer) {
    for token in lex {
        if token == TokenType::EOF {
            break;
        }
        println!("Token: {:?}", token)
    }
}

fn print_prompt() -> std::io::Result<()> {
    print!("{}", PROMPT);

    // Call flush to emit lines immediately for line buffered stdout
    stdout().flush()
}
