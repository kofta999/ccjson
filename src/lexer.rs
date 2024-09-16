#[derive(Debug, PartialEq)]
pub enum TokenType {
    OpenCurlyBrace,
    CloseCurlyBrace,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub value: String,
    pub token_type: TokenType,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    for char in input.chars() {
        // TODO: Needs change later
        let token_type = match char {
            '{' => TokenType::OpenCurlyBrace,
            '}' => TokenType::CloseCurlyBrace,
            _ => continue,
        };

        tokens.push(Token {
            value: char.to_string(),
            token_type,
        });
    }

    tokens
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_step_1() {
        let valid = fs::read_to_string("./tests/step1/valid.json").unwrap();
        let invalid = fs::read_to_string("./tests/step1/invalid.json").unwrap();

        assert_eq!(
            vec![
                Token {
                    token_type: TokenType::OpenCurlyBrace,
                    value: "{".to_string()
                },
                Token {
                    token_type: TokenType::CloseCurlyBrace,
                    value: "}".to_string()
                },
            ],
            tokenize(&valid)
        );

        assert_eq!(Vec::<Token>::new(), tokenize(&invalid));
    }
}
