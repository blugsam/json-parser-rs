use std::{vec::IntoIter, iter::Peekable};

use crate::{Value, tokenize::Token};

#[derive(Debug, PartialEq)]
pub enum TokenParseError {
    UnfinishedEscape,
    InvalidHexValue,
    InvalidCodePointValue
}

pub fn parse_tokens(tokens: &mut Peekable<IntoIter<Token>>) -> Result<Value, TokenParseError> {
    let token = tokens.next().unwrap();

    match token {
        Token::Null => Ok(Value::Null),
        Token::True => Ok(Value::Boolean(true)),    
        Token::False => Ok(Value::Boolean(false)),
        Token::Number(number) => Ok(Value::Number(number)),
        Token::String(string) => parse_string(&string),
        Token::LeftBracket => todo!(),
        Token::RightBracket => todo!(),
        _ => todo!()
    }
}

fn parse_string(input: &str) -> Result<Value, TokenParseError> {
    let mut output = String::new();

    let mut is_escaping = false;

    let mut chars = input.chars();
    while let Some(next_char) = chars.next() {
        if is_escaping {
            match next_char {
                '"' => output.push('"'),
                '\\' => output.push('\\'),
                // `\b` (backspace) is a valid escape in JSON, but not Rust
                'b' => output.push('\u{8}'),
                // `\f` (formfeed) is a valid escape in JSON, but not Rust
                'f' => output.push('\u{12}'),
                'n' => output.push('\n'),
                'r' => output.push('\r'),
                't' => output.push('\t'),
                'u' => {
                    let mut sum = 0;
                    for i in 0..4 {
                        let next_char = chars.next().ok_or(TokenParseError::UnfinishedEscape)?;
                        let digit = next_char
                            .to_digit(16)
                            .ok_or(TokenParseError::InvalidHexValue)?;
                        sum += (16u32).pow(3 - i) * digit;
                    }
                    let unescaped_char = char::from_u32(sum).ok_or(TokenParseError::InvalidCodePointValue)?;
                    output.push(unescaped_char);
                },
                // any other character *may* be escaped, ex. `\q` just push that letter `q`
                _ => output.push(next_char),
            }
            is_escaping = false;
        } 
        else if next_char == '\\' {
            is_escaping = true;
        } 
        else {
            output.push(next_char);
        }
    }

    Ok(Value::String(output))
}

#[cfg(test)]
mod tests {
    use std::iter::Peekable;
    use std::vec::IntoIter;

    use crate::tokenize::Token;
    use crate::Value;
    use super::parse_tokens;

    fn input(tokens: Vec<Token>) -> Peekable<IntoIter<Token>> {
        tokens.into_iter().peekable()
    }

    fn check(mut input: Peekable<IntoIter<Token>>, expected: Value) {
        let actual = parse_tokens(&mut input).unwrap();

        assert_eq!(actual, expected)
    }

    #[test]
    fn parses_null() {
        let input = input(vec![Token::Null]);
        let expected = Value::Null;

        check(input, expected)
    }

    #[test]
    fn parses_true() {
        let input = input(vec![Token::True]);
        let expected = Value::Boolean(true);

        check(input, expected)
    }

    #[test]
    fn parses_false() {
        let input = input(vec![Token::False]);
        let expected = Value::Boolean(false);

        check(input, expected)
    }

    #[test]
    fn parses_number() {
        let input = input(vec![Token::Number(14.0)]);
        let expected = Value::Number(14.0);

        check(input, expected);
    }

    #[test]
    fn parses_string_non_ascii() {
        let input = input(vec![Token::String("urrr madddd".to_string())]);
        let expected = Value::String(String::from("urrr madddd"));

        check(input, expected);
    }

    #[test]
    fn parses_string_with_emoji() {
        let input = input(vec![Token::String("💩💩💩💩💩".to_string())]);
        let expected = Value::String(String::from("💩💩💩💩💩"));

        check(input, expected);
    }

    #[test]
    fn parses_string_unescape_backslash() {
        let input = input(vec![Token::String(r#"hello\\world"#.into())]);
        let expected = Value::String(r#"hello\\world"#.into());

        check(input, expected);
    }
}