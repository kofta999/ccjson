#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    OpenCurlyBrace,
    CloseCurlyBrace,
    DoubleQuote,
    Colon,
    String(String),
    Comma,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars = input.chars();
    while let Some(char) = chars.next() {
        let token_type = match char {
            '{' => Token::OpenCurlyBrace,
            '}' => Token::CloseCurlyBrace,
            ':' => Token::Colon,
            ',' => Token::Comma,
            ' ' | '\n' | '\r' | '\t' => continue,
            '"' => {
                tokens.push(Token::DoubleQuote);

                let mut s = String::new();

                while let Some(c) = chars.next() {
                    if c == '"' {
                        break;
                    } else {
                        s.push(c);
                    }
                }

                tokens.push(Token::String(s));

                Token::DoubleQuote
            }
            t => {
                let err_msg = format!("Lexer Error: Unexpected Token {t}");
                return Err(err_msg);
            }
        };

        tokens.push(token_type);
    }

    Ok(tokens)
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
            vec![Token::OpenCurlyBrace, Token::CloseCurlyBrace],
            tokenize(&valid).unwrap()
        );

        assert_eq!(Vec::<Token>::new(), tokenize(&invalid).unwrap());
    }

    #[test]
    fn test_step2_lexer() {
        let valid = fs::read_to_string("./tests/step2/valid.json").unwrap();
        let invalid = fs::read_to_string("./tests/step2/invalid.json").unwrap();

        assert_eq!(
            vec![
                Token::OpenCurlyBrace,
                Token::DoubleQuote,
                Token::String(String::from("key")),
                Token::DoubleQuote,
                Token::Colon,
                Token::DoubleQuote,
                Token::String(String::from("value")),
                Token::DoubleQuote,
                Token::CloseCurlyBrace,
            ],
            tokenize(&valid).unwrap()
        );

        assert_eq!(
            vec![
                Token::OpenCurlyBrace,
                Token::DoubleQuote,
                Token::String(String::from("key")),
                Token::DoubleQuote,
                Token::Colon,
                Token::DoubleQuote,
                Token::String(String::from("value")),
                Token::DoubleQuote,
                Token::Comma,
                Token::CloseCurlyBrace,
            ],
            tokenize(&invalid).unwrap()
        );

        let valid = fs::read_to_string("./tests/step2/valid2.json").unwrap();
        let invalid = fs::read_to_string("./tests/step2/invalid2.json").unwrap();

        assert!(tokenize(&valid).is_ok());
        assert!(tokenize(&invalid).is_err());
    }
}
