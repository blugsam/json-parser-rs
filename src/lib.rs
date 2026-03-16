use std::collections::HashMap;
use crate::tokenize::{tokenize, TokenizeError};
use crate::parse::{parse_tokens, TokenParseError};

mod tokenize;
mod parse;

pub fn parse<'a>(input: &'a str) -> Result<Value, ParseError> {
    let tokens = tokenize(input)?;
    let value = parse_tokens(&mut tokens.into_iter().peekable())?;
    Ok(value)
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Null,
    Boolean(bool),
    String(String),
    Number(f64),
    Array(Vec<Value>),
    Object(HashMap<String,Value>)
}

#[cfg(test)]
impl Value {
    pub(crate) fn object<const N: usize>(pairs: [(&'static str, Self); N]) -> Self {
        let owned_pairs = pairs.map(|(key, value)| (String::from(key), value));
        let map = HashMap::from(owned_pairs);
        Self::Object(map)
    }

    pub(crate) fn string(s: &str) -> Self {
        Self::String(String::from(s))
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    TokenizeError(TokenizeError),
    ParseError(TokenParseError),
}

impl From<TokenParseError> for ParseError {
    fn from(err: TokenParseError) -> Self {
        Self::ParseError(err)
    }
}

impl From<TokenizeError> for ParseError {
    fn from(err: TokenizeError) -> Self {
        Self::TokenizeError(err)
    }
}

#[cfg(test)]
mod tests {
    use crate::{ParseError, parse};
    use crate::Value;

    fn check_valid(input: &str, expected: Value) {
        let actual = parse(input).unwrap();
        assert_eq!(actual, expected)
    }

    fn check_error<E: Into<ParseError>>(input: &str, expected: E) {
        let expected = expected.into();
        let actual = parse(input).unwrap_err();
        assert_eq!(actual, expected);
    }

    
    #[test]
    fn just_null() {
        check_valid("null", Value::Null);
    }

    #[test]
    fn just_true() {
        check_valid("true", Value::Boolean(true));
    }

    #[test]
    fn just_false() {
        check_valid("false", Value::Boolean(false));
    }

    #[test]
    fn array_with_null() {
        check_valid("[null]", Value::Array(vec![Value::Null]))
    }

    #[test]
    fn array_with_true_false() {
        check_valid(
            "[true,false]",
            Value::Array(vec![Value::Boolean(true), Value::Boolean(false)]),
        )
    }

    #[test]
    fn array_with_numbers() {
        check_valid(
            "[1, 2, 3]",
            Value::Array(vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0),
            ]),
        )
    }

    #[test]
    fn array_with_object() {
        check_valid(
            r#"[{"key": null}]"#,
            Value::Array(vec![Value::object([("key", Value::Null)])]),
        )
    }

    #[test]
    fn empty_object() {
        check_valid("{}", Value::object([]))
    }

    #[test]
    fn object_with_number() {
        check_valid(
            r#"{"key": 1}"#,
            Value::object([("key", Value::Number(1.0))]),
        );
    }

    #[test]
    fn object_with_string() {
        check_valid(
            r#"{"key": "value"}"#,
            Value::object([("key", Value::String("value".to_string()))]),
        );
    }

    #[test]
    fn object_with_null() {
        check_valid(r#"{"key": null}"#, Value::object([("key", Value::Null)]));
    }

    #[test]
    fn object_with_true() {
        check_valid(
            r#"{"key": true}"#,
            Value::object([("key", Value::Boolean(true))]),
        )
    }

    #[test]
    fn object_with_false() {
        check_valid(
            r#"{"key": false}"#,
            Value::object([("key", Value::Boolean(false))]),
        )
    }

    #[test]
    fn parse_valid() {
        check_valid(
            r#"{"name": "minecraft", "is my life": true, "version": 1.5}"#,
            Value::object([
                ("name", Value::string("minecraft")),
                ("is my life", Value::Boolean(true)),
                ("version", Value::Number(1.5))])
        );
    }

    #[test]
    fn parse_valid_complex() {
        check_valid(
            r#"{"user": {"id": 1415436218769, "tags": ["admin", "ru"]}}"#,
            Value::object([
                ("user", Value::object([
                    ("id", Value::Number(1415436218769.0)),
                    ("tags", Value::Array(vec![
                        Value::string("admin"),
                        Value::string("ru")
                    ])),
                ]
                ))
            ])
        );
    }
}
