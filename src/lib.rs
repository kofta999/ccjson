use lexer::tokenize;
use parser::Parser;

mod lexer;
pub mod parser;

pub fn run(input: &str) -> Result<(), String> {
    let tokens = match tokenize(&input) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("ERROR: {e}");
            return Err(e);
        }
    };

    let parser = Parser::new(tokens);

    match parser.parse() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("ERROR: {e}");
            return Err(e);
        }
    }

    Ok(())
}
