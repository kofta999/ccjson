use std::process;

use lexer::tokenize;
use parser::Parser;

mod lexer;
pub mod parser;

pub fn run(input: &str) {
    let tokens = tokenize(&input).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1)
    });

    let parser = Parser::new(tokens);

    parser.parse().unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1)
    });

    process::exit(0);
}
