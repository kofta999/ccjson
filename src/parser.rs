use std::cell::RefCell;

use crate::lexer::{tokenize, Token, TokenType};

pub struct Parser {
    tokens: RefCell<Box<dyn Iterator<Item = Token>>>,
}

impl Parser {
    pub fn new(input: &str) -> Parser {
        let tokens = tokenize(input);

        Parser {
            tokens: RefCell::new(Box::new(tokens.into_iter())),
        }
    }

    pub fn parse(&self) -> Result<(), &str> {
        match self.tokens.borrow_mut().next() {
            Some(t) => {
                if t.token_type == TokenType::OpenCurlyBrace {
                } else {
                    return Err("Invalid JSON");
                }
            }
            None => return Err("Invalid JSON"),
        }

        match self.tokens.borrow_mut().next() {
            Some(t) => {
                if t.token_type == TokenType::CloseCurlyBrace {
                } else {
                    return Err("Invalid JSON");
                }
            }
            None => return Err("Invalid JSON"),
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_parser_step1_valid() {
        let valid = fs::read_to_string("./tests/step1/valid.json").unwrap();

        // TODO: fix so that using mut is not needed
        let parser1 = Parser::new(&valid);

        assert!(parser1.parse().is_ok());
    }

    #[test]
    fn test_parser_step1_invalid() {
        let invalid = fs::read_to_string("./tests/step1/invalid.json").unwrap();

        let parser2 = Parser::new(&invalid);

        assert!(parser2.parse().is_err());
    }
}
