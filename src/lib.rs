use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Value {
    Null,
    Boolean(bool),
    String(String),
    Number(f64),
    Array(Vec<Value>),
    Object(HashMap<String,Value>)
}

mod tokenize;
mod parse;

#[cfg(test)]
mod tests {
    
}
