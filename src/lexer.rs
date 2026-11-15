use crate::tokens::{LexError, Token, TokenType};

pub fn lex_code(code: String) -> Result<Vec<Token>, Vec<LexError>> {
    let mut line_number: u64 = 1;
    let mut code = code.chars().peekable();
    let mut tokens = vec![];
    let mut errors = vec![];
    loop {
        match code.next() {
            Some('\n') => line_number += 1,
            Some(' ' | '\t' | '\r') => (),
            Some('>') => match code.peek() {
                Some('=') => {
                    code.next();
                    tokens.push(Token::new(TokenType::IsGreaterThanOrEqualTo, line_number));
                }
                _ => tokens.push(Token::new(TokenType::IsGreaterThan, line_number)),
            },
            Some('<') => match code.peek() {
                Some('=') => {
                    code.next();
                    tokens.push(Token::new(TokenType::IsLessThanOrEqualTo, line_number));
                }
                _ => tokens.push(Token::new(TokenType::IsLessThan, line_number)),
            },
            Some('=') => match code.peek() {
                Some('=') => {
                    code.next();
                    tokens.push(Token::new(TokenType::IsEqualTo, line_number));
                }
                _ => tokens.push(Token::new(TokenType::Equals, line_number)),
            },
            Some('!') => match code.peek() {
                Some('=') => {
                    code.next();
                    tokens.push(Token::new(TokenType::IsNotEqualTo, line_number));
                }
                _ => tokens.push(Token::new(TokenType::Exclamationmark, line_number)),
            },
            Some('+') => match code.peek() {
                Some('=') => {
                    code.next();
                    tokens.push(Token::new(TokenType::PlusEqual, line_number));
                }
                _ => tokens.push(Token::new(TokenType::Plus, line_number)),
            },
            Some('-') => match code.peek() {
                Some('=') => {
                    code.next();
                    tokens.push(Token::new(TokenType::MinusEqual, line_number));
                }
                _ => tokens.push(Token::new(TokenType::Minus, line_number)),
            },
            Some('*') => match code.peek() {
                Some('=') => {
                    code.next();
                    tokens.push(Token::new(TokenType::MultiplyEqual, line_number));
                }
                _ => tokens.push(Token::new(TokenType::Multiply, line_number)),
            },
            Some('%') => match code.peek() {
                Some('=') => {
                    code.next();
                    tokens.push(Token::new(TokenType::ModuloEqual, line_number));
                }
                _ => tokens.push(Token::new(TokenType::Modulo, line_number)),
            },
            Some('/') => match code.peek() {
                Some('/') => {
                    while code.peek().unwrap_or(&'\n') != &'\n' {
                        code.next();
                    }
                }
                Some('=') => {
                    code.next();
                    tokens.push(Token::new(TokenType::DivideEqual, line_number));
                }
                _ => tokens.push(Token::new(TokenType::Divide, line_number)),
            },
            Some('&') => match code.peek() {
                Some('&') => {
                    code.next();
                    tokens.push(Token::new(TokenType::And, line_number));
                }
                _ => errors.push(LexError::InvalidToken("&".to_string(), line_number)),
            },
            Some('|') => match code.peek() {
                Some('|') => {
                    code.next();
                    tokens.push(Token::new(TokenType::Or, line_number));
                }
                _ => errors.push(LexError::InvalidToken("|".to_string(), line_number)),
            },
            Some('(') => tokens.push(Token::new(TokenType::OpenParen, line_number)),
            Some(')') => tokens.push(Token::new(TokenType::CloseParen, line_number)),
            Some('{') => tokens.push(Token::new(TokenType::OpenBrace, line_number)),
            Some('}') => tokens.push(Token::new(TokenType::CloseBrace, line_number)),
            Some('[') => tokens.push(Token::new(TokenType::OpenSq, line_number)),
            Some(']') => tokens.push(Token::new(TokenType::CloseSq, line_number)),
            Some(';') => tokens.push(Token::new(TokenType::Semicolon, line_number)),
            Some(',') => tokens.push(Token::new(TokenType::Comma, line_number)),
            Some('?') => tokens.push(Token::new(TokenType::Questionmark, line_number)),
            Some(x) if x.is_numeric() => {
                let mut number = x.to_string();
                while code.peek().is_some_and(|c| c.is_ascii_digit() || c == &'.') {
                    if let Some(c_sanitized) = code.next() {
                        number.push(c_sanitized);
                    } else {
                        break;
                    }
                }
                if number.contains('.') {
                    if let Ok(dec) = number.parse::<f64>() {
                        tokens.push(Token::new(TokenType::Decimal(dec), line_number));
                    } else {
                        errors.push(LexError::InvalidNumber(number, line_number));
                    }
                } else {
                    if let Ok(int) = number.parse::<i64>() {
                        tokens.push(Token::new(TokenType::Number(int), line_number));
                    } else {
                        errors.push(LexError::InvalidNumber(number, line_number));
                    }
                }
            }
            Some(x) if x.is_alphabetic() || x == '_' => {
                let mut word = x.to_string();
                while code
                    .peek()
                    .is_some_and(|c| c.is_ascii_alphanumeric() || c == &'_')
                {
                    if let Some(c_sanitized) = code.next() {
                        word.push(c_sanitized);
                    } else {
                        break;
                    }
                }
                match TokenType::from_word(word) {
                    Ok(tok) => tokens.push(Token::new(tok, line_number)),
                    Err(word) => errors.push(LexError::InvalidNumber(word, line_number)),
                }
            }
            Some(x) => errors.push(LexError::InvalidToken(x.into(), line_number)),
            None => {
                tokens.push(Token::new(TokenType::Eof, line_number));
                break;
            }
        }
    }
    if errors.is_empty() {
        return Ok(tokens);
    }
    Err(errors)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokens::TokenType::*;

    #[test]
    fn times_equals() {
        assert_eq!(
            lex_code("?x *= 36;".to_string()),
            Ok(vec![
                Token::new(Questionmark, 1),
                Token::new(Identifier("x".into()), 1),
                Token::new(MultiplyEqual, 1),
                Token::new(Number(36), 1),
                Token::new(Semicolon, 1),
                Token::new(Eof, 1)
            ])
        );
    }

    #[test]
    fn lots_of_newlines() {
        assert_eq!(
            lex_code("x\ny\nz\n==\n!=\n>=\nwhile {\n}".to_string()),
            Ok(vec![
                Token::new(Identifier("x".into()), 1),
                Token::new(Identifier("y".into()), 2),
                Token::new(Identifier("z".into()), 3),
                Token::new(IsEqualTo, 4),
                Token::new(IsNotEqualTo, 5),
                Token::new(IsGreaterThanOrEqualTo, 6),
                Token::new(While, 7),
                Token::new(OpenBrace, 7),
                Token::new(CloseBrace, 8),
                Token::new(Eof, 8),
            ])
        );
    }

    #[test]
    fn lots_of_math() {
        assert_eq!(
            lex_code("?x = 36;\nx /= 3;\nx -= 2;\nx *= 3;\nx += 6;".into()),
            Ok(vec![
                Token::new(Questionmark, 1),
                Token::new(Identifier("x".into()), 1),
                Token::new(Equals, 1),
                Token::new(Number(36), 1),
                Token::new(Semicolon, 1),
                Token::new(Identifier("x".into()), 2),
                Token::new(DivideEqual, 2),
                Token::new(Number(3), 2),
                Token::new(Semicolon, 2),
                Token::new(Identifier("x".into()), 3),
                Token::new(MinusEqual, 3),
                Token::new(Number(2), 3),
                Token::new(Semicolon, 3),
                Token::new(Identifier("x".into()), 4),
                Token::new(MultiplyEqual, 4),
                Token::new(Number(3), 4),
                Token::new(Semicolon, 4),
                Token::new(Identifier("x".into()), 5),
                Token::new(PlusEqual, 5),
                Token::new(Number(6), 5),
                Token::new(Semicolon, 5),
                Token::new(Eof, 5),
            ])
        );
    }

    #[test]
    fn comparisons_without_spaces() {
        assert_eq!(
            lex_code("a==b!=c<=d>=e<f>g=!h".into()),
            Ok(vec![
                Token::new(Identifier("a".into()), 1),
                Token::new(IsEqualTo, 1),
                Token::new(Identifier("b".into()), 1),
                Token::new(IsNotEqualTo, 1),
                Token::new(Identifier("c".into()), 1),
                Token::new(IsLessThanOrEqualTo, 1),
                Token::new(Identifier("d".into()), 1),
                Token::new(IsGreaterThanOrEqualTo, 1),
                Token::new(Identifier("e".into()), 1),
                Token::new(IsLessThan, 1),
                Token::new(Identifier("f".into()), 1),
                Token::new(IsGreaterThan, 1),
                Token::new(Identifier("g".into()), 1),
                Token::new(Equals, 1),
                Token::new(Exclamationmark, 1),
                Token::new(Identifier("h".into()), 1),
                Token::new(Eof, 1),
            ])
        );
    }

    #[test]
    fn comments_and_windows_line_endings() {
        assert_eq!(
            lex_code("// start\r\nwhile x { y %= 2; }\r\n// end with no newline".into()),
            Ok(vec![
                Token::new(While, 2),
                Token::new(Identifier("x".into()), 2),
                Token::new(OpenBrace, 2),
                Token::new(Identifier("y".into()), 2),
                Token::new(ModuloEqual, 2),
                Token::new(Number(2), 2),
                Token::new(Semicolon, 2),
                Token::new(CloseBrace, 2),
                Token::new(Eof, 3),
            ])
        );
    }

    #[test]
    fn keywords_vs_identifiers() {
        assert_eq!(
            lex_code("if iffy elif else whiley true false truex".into()),
            Ok(vec![
                Token::new(If, 1),
                Token::new(Identifier("iffy".into()), 1),
                Token::new(Elif, 1),
                Token::new(Else, 1),
                Token::new(Identifier("whiley".into()), 1),
                Token::new(Bool(true), 1),
                Token::new(Bool(false), 1),
                Token::new(Identifier("truex".into()), 1),
                Token::new(Eof, 1),
            ])
        );
    }

    #[test]
    fn decimals_and_brackets() {
        assert_eq!(
            lex_code("(3.14 * [x]) - 0.5 % 10".into()),
            Ok(vec![
                Token::new(OpenParen, 1),
                Token::new(Decimal(3.14), 1),
                Token::new(Multiply, 1),
                Token::new(OpenSq, 1),
                Token::new(Identifier("x".into()), 1),
                Token::new(CloseSq, 1),
                Token::new(CloseParen, 1),
                Token::new(Minus, 1),
                Token::new(Decimal(0.5), 1),
                Token::new(Modulo, 1),
                Token::new(Number(10), 1),
                Token::new(Eof, 1),
            ])
        );
    }

    #[test]
    fn errors() {
        assert!(lex_code("x = 5 @ 3;".into()).is_err());
        assert_eq!(
            lex_code("x = 1.2.3;".into()),
            Err(vec![LexError::InvalidNumber("1.2.3".into(), 1)])
        );
    }
}
