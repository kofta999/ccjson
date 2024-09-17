use std::cell::RefCell;

use crate::lexer::{tokenize, Token, TokenType};

pub struct Parser {
    cursor: RefCell<usize>,
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            cursor: RefCell::new(0),
            tokens,
        }
    }

    fn at(&self) -> Result<&Token, &str> {
        match self.tokens.get(*self.cursor.borrow()) {
            Some(t) => Ok(t),
            None => Err("Invalid JSON"),
        }
    }

    fn eat(&self) -> Result<&Token, &str> {
        let mut cursor = self.cursor.borrow_mut();
        match self.tokens.get(*cursor) {
            Some(t) => {
                *cursor += 1;
                Ok(t)
            }
            None => Err("Invalid Token"),
        }
    }

    fn expect(&self, token_type: TokenType) -> Result<(), &str> {
        if self.eat()?.token_type != token_type {
            return Err("Unexpected Token");
        }

        Ok(())
    }

    pub fn parse(&self) -> Result<(), &str> {
        self.expect(TokenType::OpenCurlyBrace)?;
        // Do parsing for content

        if self.at()?.token_type == TokenType::DoubleQuote {
            self.parse_kv_pair()?;
        }

        self.expect(TokenType::CloseCurlyBrace)?;
        Ok(())
    }

    fn parse_kv_pair(&self) -> Result<(), &str> {
        self.eat()?;

        self.expect(TokenType::String)?;
        self.expect(TokenType::DoubleQuote)?;

        println!("here");

        self.expect(TokenType::Colon)?;

        self.expect(TokenType::DoubleQuote)?;
        self.expect(TokenType::String)?;
        self.expect(TokenType::DoubleQuote)?;

        if self.at()?.token_type == TokenType::Comma {
            self.eat()?;
            self.parse_kv_pair()?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_parser_step1() {
        let valid = fs::read_to_string("./tests/step1/valid.json").unwrap();
        let valid_tokens = tokenize(&valid).unwrap();
        let parser1 = Parser::new(valid_tokens);
        assert!(parser1.parse().is_ok());

        let invalid = fs::read_to_string("./tests/step1/invalid.json").unwrap();
        let invalid_tokens = tokenize(&invalid).unwrap();
        let parser2 = Parser::new(invalid_tokens);
        assert!(parser2.parse().is_err());
    }

    #[test]
    fn test_parser_step2() {
        let valid = fs::read_to_string("./tests/step2/valid.json").unwrap();
        let valid_tokens = tokenize(&valid).unwrap();
        let parser = Parser::new(valid_tokens);
        assert!(parser.parse().is_ok());

        let invalid = fs::read_to_string("./tests/step2/invalid.json").unwrap();
        let invalid_tokens = tokenize(&invalid).unwrap();
        let parser = Parser::new(invalid_tokens);
        assert!(parser.parse().is_err());

        let valid2 = fs::read_to_string("./tests/step2/valid2.json").unwrap();
        let valid2_tokens = tokenize(&valid2).unwrap();
        let parser = Parser::new(valid2_tokens);
        assert!(parser.parse().is_ok());

        // invalid2 won't be used as it tests the lexer only
    }
}
