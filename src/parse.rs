use std::{collections::HashMap, iter::Peekable, vec::IntoIter};

use crate::{Value, tokenize::Token};

#[derive(Debug, PartialEq)]
pub enum TokenParseError {
    UnfinishedEscape,
    InvalidHexValue,
    InvalidCodePointValue,
    ExpectedComma,
    ExpectedProperty,
    ExpectedColon
}

pub fn parse_tokens(tokens: &mut Peekable<IntoIter<Token>>) -> Result<Value, TokenParseError> {
    let token = tokens.next().unwrap();

    match token {
        Token::Null => Ok(Value::Null),
        Token::True => Ok(Value::Boolean(true)),    
        Token::False => Ok(Value::Boolean(false)),
        Token::Number(number) => Ok(Value::Number(number)),
        Token::String(string) => parse_string(&string),
        Token::LeftBracket => parse_array(tokens),
        Token::LeftBrace => parse_objects(tokens),
        _ => todo!()
    }
}

fn parse_string(input: &str) -> Result<Value, TokenParseError> {
    let unescaped = unescape_string(input)?;
    Ok(Value::String(unescaped))
}

fn unescape_string(input: &str) -> Result<String, TokenParseError> {
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

    Ok(output)
}

fn parse_array(tokens: &mut Peekable<IntoIter<Token>>) -> Result<Value, TokenParseError> {
    let mut array: Vec<Value> = Vec::new();

    loop {
        if *tokens.peek().unwrap() == Token::RightBracket {
            break;
        }
        
        let value = parse_tokens(tokens)?;
        array.push(value);
        
        let token = tokens.next().unwrap();
        match token {
            Token::Comma => continue,
            Token::RightBracket => return Ok(Value::Array(array)),
            _ => return Err(TokenParseError::ExpectedComma),
        }
    }

    tokens.next();

    Ok(Value::Array(array))
}

fn parse_objects(tokens: &mut Peekable<IntoIter<Token>>) -> Result<Value, TokenParseError> {
    let mut map = HashMap::new();

    loop {
        if let Some(&Token::RightBrace) = tokens.peek() {
            break;
        }

        if let Some(Token::String(s)) = tokens.next() {
            if let Some(Token::Colon) = tokens.next() {
                let key = unescape_string(&s)?;
                let value = parse_tokens(tokens)?;
                map.insert(key, value);
            } else {
                return Err(TokenParseError::ExpectedColon)
            }
        } else {
            return Err(TokenParseError::ExpectedProperty)
        }

        match tokens.peek() {
            Some(Token::Comma) => {
                tokens.next();
            }
            Some(Token::RightBrace) => {
                break;
            }
            _ => return Err(TokenParseError::ExpectedComma)
        }
    }

    tokens.next();

    Ok(Value::Object(map))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
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

    #[test]
    fn parses_array_one_element() {
        let input = input(vec![Token::LeftBracket, Token::True, Token::RightBracket]);
        let expected = Value::Array(vec![Value::Boolean(true)]);

        check(input, expected);
    }

    #[test]
    fn parses_array_two_elements() {
        let input = input(vec![Token::LeftBracket, Token::Null, Token::Comma, Token::Number(16.0), Token::RightBracket]);
        let expected = Value::Array(vec![Value::Null, Value::Number(16.0)]);

        check(input, expected)
    }

    #[test]
    fn parses_empty_array() {
        let input = input(vec![Token::LeftBracket, Token::RightBracket]);
        let expected = Value::Array(vec![]);

        check(input, expected)
    }

    #[test]
    fn parse_nested_array() {
        let input = input(vec![
            Token::LeftBracket,
            Token::Null,
            Token::Comma, 
            Token::Number(16.0),
            Token::Comma,
            Token::LeftBracket, 
            Token::Null,
            Token::Comma,
            Token::Number(16.0),
            Token::RightBracket,
            Token::Comma,
            Token::Null,
            Token::RightBracket]
        );
        
        let expected = Value::Array(vec![
            Value::Null,
            Value::Number(16.0),
            Value::Array(vec![
                Value::Null,
                Value::Number(16.0)
            ]),
            Value::Null
        ]);

        check(input, expected);
    }

    #[test]
    fn parse_empty_object() {
        let input = input(vec![Token::LeftBrace, Token::RightBrace]);
        let expected = Value::Object(HashMap::new());

        check(input, expected)
    }

    #[test]
    fn parse_object() {
        let input = input(vec![
            Token::LeftBrace, 
            Token::String("ASPNETCORE_ENVIRONMENT".into()), 
            Token::Colon, 
            Token::String("Development".into()),
            Token::RightBrace]
        );
        
        let mut map = HashMap::new();
        map.insert(
            "ASPNETCORE_ENVIRONMENT".into(),
            Value::String("Development".into())
        );

        let expected = Value::Object(map);

        check(input, expected)
    }

    #[test]
    fn parse_object_with_escaped_chars() {
        let input = input(vec![
            Token::LeftBrace,
            Token::String("key".to_string()),
            Token::Colon,
            Token::String("value with \\\"quotes\\\" and \\n newline".to_string()), 
            Token::RightBrace]
        );

        let mut map = HashMap::new();
        map.insert(
            "key".to_string(), 
            Value::String("value with \"quotes\" and \n newline".to_string())
        );

        let expected = Value::Object(map);

        check(input, expected);
    }
}