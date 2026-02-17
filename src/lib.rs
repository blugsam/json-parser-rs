use std::collections::HashMap;
use crate::tokenize::{tokenize, TokenizeError};
use crate::parse::{parse_tokens, TokenParseError};

mod tokenize;
mod parse;

pub fn parse(input: String) -> Result<Value, ParseError> {
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
    use crate::parse;
    use std::collections::HashMap;
    use crate::Value;

    #[test]
    fn parse_valid() {
        let input = r#"{"name": "Minecraft", "isMyLife": true, "version": 1.5}"#.to_string();
        
        let result = parse(input).unwrap();
        
        let mut expected_map = HashMap::new();
        expected_map.insert("name".to_string(), Value::String("Minecraft".to_string()));
        expected_map.insert("isMyLife".to_string(), Value::Boolean(true));
        expected_map.insert("version".to_string(), Value::Number(1.5));
        
        assert_eq!(result, Value::Object(expected_map));
    }

    #[test]
    fn parse_valid_nested_structure() {
        let input = r#"{"user": {"id": 1415436218769, "tags": ["admin", "ru"]}}"#.to_string();
        
        let result = parse(input).unwrap();

        let mut user_map = HashMap::new();
        user_map.insert("id".to_string(), Value::Number(1415436218769.0));
        user_map.insert(
            "tags".to_string(), 
            Value::Array(vec![
                Value::String("admin".to_string()),
                Value::String("ru".to_string())]
            )
        );

        let mut root_map = HashMap::new();
        root_map.insert("user".to_string(), Value::Object(user_map));

        let expected = Value::Object(root_map);

        assert_eq!(result, expected);
    }
}
