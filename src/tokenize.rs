use std::{char, str::Chars, iter::Peekable, num::ParseFloatError};

#[derive(Debug, PartialEq)]
pub enum Token {
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
    String(String)
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenizeError {
    UnfinishedLiteralValue,
    InvalidNumber(String),
    ParseNumberError(ParseFloatError),
    UnclosedQuotes,
    CharNotRecognized(char),
    UnexpectedEof
}

pub fn tokenize(input: String) -> Result<Vec<Token>, TokenizeError> {
    let mut chars = input.chars().peekable();

    let mut tokens = Vec::new();

    while let Some(c) = chars.next() {
        let token = make_token(&mut chars,c)?;
        tokens.push(token);
    }
    
    Ok(tokens)
}

fn make_token(chars: &mut Peekable<Chars<'_>>, mut ch: char) -> Result<Token, TokenizeError> {
    while ch.is_ascii_whitespace() {
        if chars.peek() == None {
            return Err(TokenizeError::UnexpectedEof);
        }
        ch = chars.next().unwrap();
    }

    let token = match ch {
        c if is_number(ch) => tokenize_float(chars, c)?,
        '"' => tokenize_string(chars)?,
        '[' => Token::LeftBracket,
        ']' => Token::RightBracket,
        '{' => Token::LeftBrace,
        '}' => Token::RightBrace,
        ',' => Token::Comma,
        ':' => Token::Colon,
        't' => tokenize_true(chars)?,
        'f' => tokenize_false(chars)?,
        'n' => tokenize_null(chars)?,
        ch => return Err(TokenizeError::CharNotRecognized(ch)),
    };

    Ok(token)
}

fn is_number(ch: char) -> bool {
    match ch {
        '-' => true,
        ch if ch.is_ascii_digit() => true,
        _ => false
    }
}

fn tokenize_float(chars: &mut Peekable<Chars<'_>>, ch: char) -> Result<Token, TokenizeError> {
    let mut unparsed_num = String::new();
    unparsed_num.push(ch);

    if ch == '-' {
        if chars.peek().is_some_and(|&c| c == '0') {
            unparsed_num.push(chars.next().unwrap());

            if chars.peek().is_some_and(|&c| c.is_ascii_digit()) {
                return  Err(TokenizeError::InvalidNumber("Invalid number provided.".to_string()));
            }
        }
    }

    if ch == '0' {
        if chars.peek().is_some_and(|&c| c.is_ascii_digit()) {
            return Err(TokenizeError::InvalidNumber("Invalid number provided.".to_string()));
        } 
    }

    let mut has_decimal = false;
    let mut has_exponent = false;

    while let Some(&c) = chars.peek() {
        match c {
            c if c.is_ascii_digit() => unparsed_num.push(chars.next().unwrap()),
            c if is_exponenta(has_exponent, c, chars) => {
                unparsed_num.push(chars.next().unwrap());
                has_exponent = true;
                
                if chars.peek().is_some_and(|&c| c == '+' || c == '-' ) {
                    unparsed_num.push(chars.next().unwrap());
                }

                if !chars.peek().is_some_and(|c| c.is_ascii_digit()) {
                    return Err(TokenizeError::InvalidNumber("Invalid number provided.".to_string()));
                }
            },
            c if is_decimal(has_decimal, has_exponent, c) => {
                unparsed_num.push('.');
                has_decimal = true;
                chars.next();
            }
            _ => break,
        }
    }

    match unparsed_num.parse::<f64>() {
        Ok(f) => Ok(Token::Number(f)),
        Err(e) => Err(TokenizeError::ParseNumberError(e))
    }
}

fn is_exponenta(has_exponent: bool, c: char, chars: &mut Peekable<Chars<'_>>) -> bool {
    !has_exponent && matches!(c, 'e' | 'E') && chars.peek().is_some()
}

fn is_decimal(has_decimal: bool, has_exponenta: bool, c: char) -> bool {
    c == '.' && !has_decimal && !has_exponenta
}

fn tokenize_string(chars: &mut Peekable<Chars<'_>>) -> Result<Token, TokenizeError> {
    let mut string = String::new();
    let mut is_closed: bool = false;
    let mut is_escaping: bool = false;

    while let Some(c) = chars.next() {
        match c {
            '"' if !is_escaping => { is_closed = true; break; },
            '\\' => is_escaping = !is_escaping,
            _ => is_escaping = false
        }

        string.push(c);
    }

    if !is_closed {
        return Err(TokenizeError::UnclosedQuotes);
    }

    Ok(Token::String(string))
}

fn tokenize_true(chars: &mut Peekable<Chars<'_>>) -> Result<Token, TokenizeError> {
    for expected_char in "rue".chars() {
        if chars.peek() != Some(&expected_char) {
            return Err(TokenizeError::UnfinishedLiteralValue)
        }
        chars.next();
    }

    Ok(Token::True)
}

fn tokenize_false(chars: &mut Peekable<Chars<'_>>) -> Result<Token, TokenizeError> {
    for expected_char in "alse".chars() {
        if chars.peek() != Some(&expected_char) {
            return Err(TokenizeError::UnfinishedLiteralValue)
        }
        chars.next();
    }

    Ok(Token::False)
}

fn tokenize_null(chars: &mut Peekable<Chars<'_>>) -> Result<Token, TokenizeError> {
    for expected_char in "ull".chars() {
        if chars.peek() != Some(&expected_char) {
            return Err(TokenizeError::UnfinishedLiteralValue);
        }
        chars.next();
    }

    Ok(Token::Null)
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

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected)
    }

    #[test]
    fn negative_integer() {
        let input = String::from("-123");
        let expected = [Token::Number(-123.0)];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected)
    }

        #[test]
    fn double_negative_integer() {
        let input = String::from("--123");
        let expected_error = input.parse::<f64>().unwrap_err();
        let expected = TokenizeError::ParseNumberError(expected_error);

        let actual = tokenize(input).unwrap_err();

        assert_eq!(actual, expected)
    }

    #[test]
    fn double_zero() {
        let input = String::from("00");
        let expected = TokenizeError::InvalidNumber("Invalid number provided.".to_string());

        let actual = tokenize(input).unwrap_err();

        assert_eq!(actual, expected)
    }

    #[test]
    fn neagtive_double_zero() {
        let input = String::from("-00");
        let expected = TokenizeError::InvalidNumber("Invalid number provided.".to_string());

        let actual = tokenize(input).unwrap_err();

        assert_eq!(actual, expected)
    }

    // string
    #[test]
    fn string() {
        let input = String::from("\"string\"");
        let expected = [Token::String("string".to_string())];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected)
    }

    #[test]
    fn unclosed_quotes() {
        let input = String::from("\"string");
        let expected = TokenizeError::UnclosedQuotes;

        let actual = tokenize(input).unwrap_err();

        assert_eq!(actual, expected)
    }

    // decimal
    #[test]
    fn decimal() {
        let input = String::from("0.88");
        let expected = [Token::Number(0.88)];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected)
    }

    #[test]
    fn negative_decimal() {
        let input = String::from("-0.88");
        let expected = [Token::Number(-0.88)];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected)
    }

    // exponent
    #[test]
    fn exponent() {
        let input = String::from("0.5e2");
        let expected = [Token::Number(0.5e2)];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected)
    }

    #[test]
    fn negative_exponent() {
        let input = String::from("-0.5e2");
        let expected = [Token::Number(-0.5e2)];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected)
    }

    // punctuation
    #[test]
    fn just_comma() {
        let input = String::from(",");
        let expected = [Token::Comma];

        let actual = tokenize(input).unwrap();

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

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }

    // bool
    #[test]
    fn just_null() {
        let input = String::from("null");
        let expected = [Token::Null];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn just_true() {
        let input = String::from("true");
        let expected = [Token::True];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn just_false() {
        let input = String::from("false");
        let expected = [Token::False];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn true_comma() {
        let input = String::from("true,");
        let expected = [Token::True, Token::Comma];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }
}