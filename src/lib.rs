use lexer::tokenize;
use parser::Parser;

mod lexer;
pub mod parser;

pub fn run(input: &str) -> Result<(), String> {
    let tokens = tokenize(&input)?;
    let parser = Parser::new(tokens);

    parser.parse()?;

    Ok(())
}
