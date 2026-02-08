use std::char;
use std::iter::Peekable;
use std::str::Chars;

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

fn make_token(chars: &mut Peekable<Chars<'_>>, ch: char) -> Result<Token, TokenizeError> {
    let token = match ch {
        '[' => Token::LeftBracket,
        ']' => Token::RightBracket,
        '{' => Token::LeftBrace,
        '}' => Token::RightBrace,
        ',' => Token::Comma,
        ':' => Token::Colon,
        't' => tokenize_true(chars)?,
        'f' => tokenize_false(chars)?,
        'n' => tokenize_null(chars)?,
        _ => todo!("implement other tokens")
    };

    Ok(token)
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
    use super::{tokenize, Token};

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