#[derive(Debug, PartialEq)]
pub enum Token {
    // Symbols
    OpenCurlyBrace,
    CloseCurlyBrace,
    OpenBracket,
    CloseBracket,
    Colon,
    Comma,

    // Types
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(char) = chars.next() {
        let token_type = match char {
            '{' => Token::OpenCurlyBrace,
            '}' => Token::CloseCurlyBrace,
            ':' => Token::Colon,
            ',' => Token::Comma,
            '[' => Token::OpenBracket,
            ']' => Token::CloseBracket,
            ' ' | '\n' | '\r' | '\t' => continue,
            '"' => {
                let mut s = String::new();
                let allowed_escapes = ['"', '\\', '/', 'b', 'f', 'n', 'r', 't', 'u'];
                let invalid_escapes = ['\t', '\n'];

                while let Some(c) = chars.next() {
                    let peek = match chars.peek() {
                        Some(v) => v,
                        None => break,
                    };

                    if c == '"' {
                        break;
                    }

                    if invalid_escapes.contains(&c) {
                        return Err("Lexer Error: Unescaped Characters".into());
                    }

                    if c.is_ascii() && c != '\\' {
                        s.push(c);
                    } else if c.is_ascii() && c == '\\' && allowed_escapes.contains(peek) {
                        s.push(c);
                        s.push(*peek);
                        chars.next();
                    } else {
                        return Err("Lexer Error: Illigal backslash escape".into());
                    }
                }

                Token::String(s)
            }
            '-' | '0'..='9' => {
                let mut s = String::from(char);
                let mut has_frac = false;
                let mut has_e = false;

                // If '-' then it's a negative, won't do anything
                // If '0' then it MUST be a fraction (followed by a .) or only 0
                // else a normal number (check for E / e)

                if char == '0' {
                    if let Some(c) = chars.peek() {
                        if c.is_numeric() {
                            return Err(
                                "Lexer Error: Numbers should never start with 0 except zero".into(),
                            );
                        }
                    }
                }
                while let Some(c) = chars.peek() {
                    if *c == '.' {
                        if has_frac {
                            return Err("Lexer Error: There only should be 1 decimal point".into());
                        }
                        has_frac = true;

                        s.push(*c);
                        chars.next();
                    } else if *c == 'e' || *c == 'E' {
                        if has_e {
                            return Err("Lexer Error: There only should be 1 exponential".into());
                        }
                        has_e = true;

                        // Push e | E and go to next char
                        s.push(*c);
                        chars.next();

                        let mut next = chars.next().expect("alo");

                        if next == '+' || next == '-' {
                            s.push(next);
                            next = chars.next().expect("alo");
                        }

                        if !next.is_numeric() {
                            return Err(
                                    "Lexer Error: There must be a + or - sign or a number after exponential".into(),
                                );
                        }

                        s.push(next);
                    } else if !c.is_numeric() {
                        break;
                    } else {
                        s.push(*c);
                        chars.next();
                    }
                }

                Token::Number(s.parse::<f64>().expect("This should never be reached"))
            }
            t => {
                let mut s = String::from(t);

                while let Some(c) = chars.peek() {
                    if !c.is_alphabetic() {
                        break;
                    } else {
                        s.push(*c);
                        chars.next();
                    }
                }

                match s.as_str() {
                    "true" => Token::Boolean(true),
                    "false" => Token::Boolean(false),
                    "null" => Token::Null,
                    t => {
                        let err_msg = format!("Lexer Error: Unexpected Token {t}");
                        return Err(err_msg);
                    }
                }
            }
        };

        // println!("{:?}", tokens);
        tokens.push(token_type);
    }

    println!("{:?}", tokens);
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
                Token::String(String::from("key")),
                Token::Colon,
                Token::String(String::from("value")),
                Token::CloseCurlyBrace,
            ],
            tokenize(&valid).unwrap()
        );

        assert_eq!(
            vec![
                Token::OpenCurlyBrace,
                Token::String(String::from("key")),
                Token::Colon,
                Token::String(String::from("value")),
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

    #[test]
    fn test_lexer_step3() {
        let valid = fs::read_to_string("./tests/step3/valid.json").unwrap();
        let invalid = fs::read_to_string("./tests/step3/invalid.json").unwrap();

        let expected_tokens = vec![
            Token::OpenCurlyBrace,
            Token::String(String::from("key1")),
            Token::Colon,
            Token::Boolean(true),
            Token::Comma,
            Token::String(String::from("key2")),
            Token::Colon,
            Token::Boolean(false),
            Token::Comma,
            Token::String(String::from("key3")),
            Token::Colon,
            Token::Null,
            Token::Comma,
            Token::String(String::from("key4")),
            Token::Colon,
            Token::String(String::from("value")),
            Token::Comma,
            Token::String(String::from("key5")),
            Token::Colon,
            Token::Number(101.0),
            Token::CloseCurlyBrace,
        ];

        assert_eq!(expected_tokens, tokenize(&valid).unwrap());

        assert!(tokenize(&invalid).is_err());
    }

    #[test]
    fn test_lexer_step4() {
        let valid = r#"
        {
            "key": "value",
            "key-n": 101,
            "key-o": {
                "inner key": "inner value"
            }
        }
        "#;

        let expected_tokens = vec![
            Token::OpenCurlyBrace,
            Token::String(String::from("key")),
            Token::Colon,
            Token::String(String::from("value")),
            Token::Comma,
            Token::String(String::from("key-n")),
            Token::Colon,
            Token::Number(101.0),
            Token::Comma,
            Token::String(String::from("key-o")),
            Token::Colon,
            Token::OpenCurlyBrace,
            Token::String(String::from("inner key")),
            Token::Colon,
            Token::String(String::from("inner value")),
            Token::CloseCurlyBrace,
            Token::CloseCurlyBrace,
        ];

        assert_eq!(expected_tokens, tokenize(valid).unwrap());
    }
}
