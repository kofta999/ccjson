use crate::lexer::Token;
use std::{cell::RefCell, mem};

pub struct Parser {
    cursor: RefCell<usize>,
    tokens: Vec<Token>,
}

fn eq_enum<T>(a: &T, b: &T) -> bool {
    mem::discriminant(a) == mem::discriminant(b)
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

    fn expect(&self, token_type: Token) -> Result<(), &str> {
        if !eq_enum(self.eat()?, &token_type) {
            return Err("Unexpected Token");
        }

        Ok(())
    }

    pub fn parse(&self) -> Result<(), &str> {
        self.expect(Token::OpenCurlyBrace)?;
        // Do parsing for content
        self.parse_inside()?;

        self.expect(Token::CloseCurlyBrace)?;
        Ok(())
    }

    fn parse_inside(&self) -> Result<(), &str> {
        if eq_enum(self.at()?, &Token::String(String::new())) {
            println!("tes");
            self.parse_kv_pair()?;
        }

        Ok(())
    }

    fn parse_kv_pair(&self) -> Result<(), &str> {
        self.expect(Token::String(String::new()))?;

        self.expect(Token::Colon)?;

        match self.eat()? {
            Token::String(_) => (),
            Token::Number(_) => (),
            Token::Boolean(_) => (),
            Token::Null => (),
            Token::OpenCurlyBrace => {
                self.parse_inside()?;
                self.expect(Token::CloseCurlyBrace)?;
            }
            _ => return Err("Unexpected token"),
        }

        if self.at()? == &Token::Comma {
            self.eat()?;
            self.parse_kv_pair()?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenize;
    use std::fs;

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

    #[test]
    fn test_parser_step3() {
        let valid = fs::read_to_string("./tests/step3/valid.json").unwrap();
        let valid_tokens = tokenize(&valid).unwrap();
        let parser = Parser::new(valid_tokens);
        assert!(parser.parse().is_ok());

        // Invalid won't be used as it tests the lexer only
        // let invalid = fs::read_to_string("./tests/step3/invalid.json").unwrap();
        // let invalid_tokens = tokenize(&invalid).unwrap();
        // let parser = Parser::new(invalid_tokens);
        // assert!(parser.parse().is_err());
    }

    #[test]
    fn test_parser_step4() {
        let valid = fs::read_to_string("./tests/step4/valid.json").unwrap();
        let valid_tokens = tokenize(&valid).unwrap();
        let parser = Parser::new(valid_tokens);
        let s = parser.parse();
        s.unwrap();
        assert!(s.is_ok());
    }
}
