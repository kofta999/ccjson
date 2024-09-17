#[derive(Debug, PartialEq)]
pub enum TokenType {
    OpenCurlyBrace,
    CloseCurlyBrace,
    DoubleQuote,
    Colon,
    String,
    Comma,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub value: String,
    pub token_type: TokenType,
}

fn create_token(value: String, token_type: TokenType) -> Token {
    Token { value, token_type }
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars = input.chars();
    while let Some(char) = chars.next() {
        let token_type = match char {
            '{' => TokenType::OpenCurlyBrace,
            '}' => TokenType::CloseCurlyBrace,
            '"' => {
                tokens.push(Token {
                    value: '"'.to_string(),
                    token_type: TokenType::DoubleQuote,
                });

                let mut s = String::new();

                while let Some(c) = chars.next() {
                    if c == '"' {
                        break;
                    } else {
                        s.push(c);
                    }
                }

                tokens.push(create_token(s, TokenType::String));

                TokenType::DoubleQuote
            }
            ':' => TokenType::Colon,
            ',' => TokenType::Comma,
            _ => continue,
        };

        tokens.push(create_token(char.to_string(), token_type));
    }

    tokens
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_step_1_lexer() {
        let valid = fs::read_to_string("./tests/step1/valid.json").unwrap();
        let invalid = fs::read_to_string("./tests/step1/invalid.json").unwrap();

        assert_eq!(
            vec![
                create_token("{".to_string(), TokenType::OpenCurlyBrace),
                create_token("}".to_string(), TokenType::CloseCurlyBrace),
            ],
            tokenize(&valid)
        );

        assert_eq!(Vec::<Token>::new(), tokenize(&invalid));
    }

    #[test]
    fn test_step2_lexer() {
        let valid = fs::read_to_string("./tests/step2/valid.json").unwrap();
        let invalid = fs::read_to_string("./tests/step2/invalid.json").unwrap();

        assert_eq!(
            vec![
                create_token("{".to_string(), TokenType::OpenCurlyBrace),
                create_token('"'.to_string(), TokenType::DoubleQuote),
                create_token("key".to_string(), TokenType::String),
                create_token('"'.to_string(), TokenType::DoubleQuote),
                create_token(":".to_string(), TokenType::Colon),
                create_token('"'.to_string(), TokenType::DoubleQuote),
                create_token("value".to_string(), TokenType::String),
                create_token('"'.to_string(), TokenType::DoubleQuote),
                create_token("}".to_string(), TokenType::CloseCurlyBrace),
            ],
            tokenize(&valid)
        );

        assert_eq!(
            vec![
                create_token("{".to_string(), TokenType::OpenCurlyBrace),
                create_token('"'.to_string(), TokenType::DoubleQuote),
                create_token("key".to_string(), TokenType::String),
                create_token('"'.to_string(), TokenType::DoubleQuote),
                create_token(":".to_string(), TokenType::Colon),
                create_token('"'.to_string(), TokenType::DoubleQuote),
                create_token("value".to_string(), TokenType::String),
                create_token('"'.to_string(), TokenType::DoubleQuote),
                create_token(",".to_string(), TokenType::Comma),
                create_token("}".to_string(), TokenType::CloseCurlyBrace),
            ],
            tokenize(&invalid)
        );
    }
}
