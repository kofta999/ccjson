use std::process;

use parser::Parser;

mod lexer;
pub mod parser;

pub fn run(input: &str) {
    let parser = Parser::new(input);

    parser.parse().unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1)
    });

    process::exit(0);
}
