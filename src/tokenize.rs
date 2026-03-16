use std::{char, num::ParseFloatError};

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    /// `{`
    LeftBrace,
    /// `}`
    RightBrace,
    /// `[`
    LeftBracket,
    /// `]`
    RightBracket,
    /// `,`
    Comma,
    /// `:`
    Colon,
    /// `null`
    Null,
    /// `false`
    False,
    /// `true`
    True,
    /// Any number literal
    Number(f64),
    /// Key of the key/value pair of string value
    String(&'a str)
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenizeError {
    UnfinishedLiteralValue,
    InvalidNumber(String),
    ParseNumberError(ParseFloatError),
    UnclosedQuotes,
    CharNotRecognized(char),
    UnexpectedCharacter,
    UnexpectedEof,
}

pub fn tokenize<'a>(input: &'a str) -> Result<Vec<Token<'a>>, TokenizeError> {
    let mut tokens = Vec::new();
    let mut current_input = input.trim_start();

    while !current_input.is_empty() {
        let (token, remaining) = make_token(current_input)?;
        
        tokens.push(token);
        
        current_input = remaining.trim_start();
    }
    
    Ok(tokens)
}

fn make_token<'a>(input: &'a str) -> Result<(Token<'a>, &'a str), TokenizeError> {
    let ch = input.chars().next().ok_or(TokenizeError::UnexpectedEof)?;

    let (token, next_pos) = match ch {
        c if is_number(ch) => tokenize_float(input)?,
        '"' => tokenize_string(input)?,
        '[' => (Token::LeftBracket, &input[1..]),
        ']' => (Token::RightBracket, &input[1..]),
        '{' => (Token::LeftBrace, &input[1..]),
        '}' => (Token::RightBrace, &input[1..]),
        ',' => (Token::Comma, &input[1..]),
        ':' => (Token::Colon, &input[1..]),
        't' => tokenize_true(input)?,
        'f' => tokenize_false(input)?,
        'n' => tokenize_null(input)?,
        _ => return Err(TokenizeError::CharNotRecognized(ch)),
    };

    Ok((token, next_pos))
}

fn is_number(ch: char) -> bool {
    match ch {
        '-' => true,
        ch if ch.is_ascii_digit() => true,
        _ => false
    }
}

fn tokenize_float<'a>(input: &'a str) -> Result<(Token<'a>, &'a str), TokenizeError> {
    let bytes = input.as_bytes();
    let mut pos = 0;

    if pos < bytes.len() && bytes[pos] == b'-' {
        pos += 1;
    }

    match bytes.get(pos) {
        Some(&b'0') => {
            pos += 1;
            if bytes.get(pos).map_or(false, |b| b.is_ascii_digit()) {
                return Err(TokenizeError::InvalidNumber("Invalid number provided.".to_string()));
            }
        }
        Some(&b) if b.is_ascii_digit() => {
            while let Some(&b) = bytes.get(pos) {
                if !b.is_ascii_digit() { break; }
                pos += 1;
            }
        }
        Some(_) => return Err(TokenizeError::UnexpectedCharacter),
        None => return Err(TokenizeError::UnexpectedEof),
    }

    if pos < bytes.len() && bytes[pos] == b'.' {
        pos += 1;
        let mut has_fraction = false;
        
        while pos < bytes.len() && bytes[pos].is_ascii_digit() {
            has_fraction = true;
            pos += 1;
        }
        
        if !has_fraction {
            return Err(TokenizeError::InvalidNumber("Invalid number provided.".to_string()));
        }
    }

    if pos < bytes.len() && (bytes[pos] == b'e' || bytes[pos] == b'E') {
        pos += 1;
        
        if pos < bytes.len() && (bytes[pos] == b'+' || bytes[pos] == b'-') {
            pos += 1;
        }
        
        let mut has_exponent = false;
        while pos < bytes.len() && bytes[pos].is_ascii_digit() {
            has_exponent = true;
            pos += 1;
        }
        
        if !has_exponent {
            return Err(TokenizeError::InvalidNumber("Invalid number provided.".to_string()))
        }
    }

    let num_slice = &input[..pos];
    let remaining = &input[pos..];

    match num_slice.parse::<f64>() {
        Ok(f) => Ok((Token::Number(f), remaining)),
        Err(e) => Err(TokenizeError::ParseNumberError(e))
    }
}

fn tokenize_string<'a>(input: &'a str) -> Result<(Token<'a>, &'a str), TokenizeError> {
    let content = &input[1..];
    
    let end = content.find('"')
        .ok_or(TokenizeError::UnclosedQuotes)?;
        
    let token = Token::String(&content[..end]);
    
    let remaining = &content[end + 1..];
    
    Ok((token, remaining))
}

fn tokenize_true<'a>(input: &'a str) -> Result<(Token<'a>, &'a str), TokenizeError> {
    const LITERAL: &str = "true";

    if !input.starts_with(LITERAL) {
        return Err(TokenizeError::UnfinishedLiteralValue);
    }

    let remaining = &input[LITERAL.len()..];

    Ok((Token::True, remaining))
}

fn tokenize_false<'a>(input: &'a str) -> Result<(Token<'a>, &'a str), TokenizeError> {
    const LITERAL: &str = "false";

    if !input.starts_with(LITERAL) {
        return Err(TokenizeError::UnfinishedLiteralValue);
    }

    let remaining = &input[LITERAL.len()..];

    Ok((Token::False, remaining))
}

fn tokenize_null<'a>(input: &'a str) -> Result<(Token<'a>, &'a str), TokenizeError> {
    const LITERAL: &str = "null";

    if !input.starts_with(LITERAL) {
        return Err(TokenizeError::UnfinishedLiteralValue);
    }

    let remaining = &input[LITERAL.len()..];

    Ok((Token::Null, remaining))
}

#[cfg(test)]
mod tests {
    use crate::tokenize::TokenizeError;

    use super::{tokenize, Token};

    // int
    #[test]
    fn integer() {
        let input = String::from("123");
        let expected = [Token::Number(123.0)];

        let actual = tokenize(&input).unwrap();

        assert_eq!(actual, expected)
    }

    #[test]
    fn negative_integer() {
        let input = String::from("-123");
        let expected = [Token::Number(-123.0)];

        let actual = tokenize(&input).unwrap();

        assert_eq!(actual, expected)
    }

    #[test]
    fn double_negative_integer() {
        let input = String::from("--123");
        
        let expected = TokenizeError::UnexpectedCharacter; 
        let actual = tokenize(&input).unwrap_err();

        assert_eq!(actual, expected)
    }

    #[test]
    fn double_zero() {
        let input = String::from("00");
        let expected = TokenizeError::InvalidNumber("Invalid number provided.".to_string());

        let actual = tokenize(&input).unwrap_err();

        assert_eq!(actual, expected)
    }

    #[test]
    fn neagtive_double_zero() {
        let input = String::from("-00");
        let expected = TokenizeError::InvalidNumber("Invalid number provided.".to_string());

        let actual = tokenize(&input).unwrap_err();

        assert_eq!(actual, expected)
    }

    // string
    #[test]
    fn string() {
        let input = String::from("\"string\"");
        let expected = [Token::String("string")];

        let actual = tokenize(&input).unwrap();

        assert_eq!(actual, expected)
    }

    #[test]
    fn unclosed_quotes() {
        let input = String::from("\"string");
        let expected = TokenizeError::UnclosedQuotes;

        let actual = tokenize(&input).unwrap_err();

        assert_eq!(actual, expected)
    }

    // decimal
    #[test]
    fn decimal() {
        let input = String::from("0.88");
        let expected = [Token::Number(0.88)];

        let actual = tokenize(&input).unwrap();

        assert_eq!(actual, expected)
    }

    #[test]
    fn negative_decimal() {
        let input = String::from("-0.88");
        let expected = [Token::Number(-0.88)];

        let actual = tokenize(&input).unwrap();

        assert_eq!(actual, expected)
    }

    // exponent
    #[test]
    fn exponent() {
        let input = String::from("0.5e2");
        let expected = [Token::Number(0.5e2)];

        let actual = tokenize(&input).unwrap();

        assert_eq!(actual, expected)
    }

    #[test]
    fn negative_exponent() {
        let input = String::from("-0.5e2");
        let expected = [Token::Number(-0.5e2)];

        let actual = tokenize(&input).unwrap();

        assert_eq!(actual, expected)
    }

    // punctuation
    #[test]
    fn just_comma() {
        let input = String::from(",");
        let expected = [Token::Comma];

        let actual = tokenize(&input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn all_punctuation() {
        let input = String::from("[{]},:");
        let expected = [
            Token::LeftBracket,
            Token::LeftBrace,
            Token::RightBracket,
            Token::RightBrace,
            Token::Comma,
            Token::Colon,
        ];

        let actual = tokenize(&input).unwrap();

        assert_eq!(actual, expected);
    }

    // bool
    #[test]
    fn just_null() {
        let input = String::from("null");
        let expected = [Token::Null];

        let actual = tokenize(&input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn just_true() {
        let input = String::from("true");
        let expected = [Token::True];

        let actual = tokenize(&input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn just_false() {
        let input = String::from("false");
        let expected = [Token::False];

        let actual = tokenize(&input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn true_comma() {
        let input = String::from("true,");
        let expected = [Token::True, Token::Comma];

        let actual = tokenize(&input).unwrap();

        assert_eq!(actual, expected);
    }
}