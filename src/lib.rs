use std::collections::HashMap;

pub enum Value {
    Null,
    Boolean(bool),
    String(String),
    Number(f64),
    Array(Vec<Value>),
    Object(HashMap<String,Value>)
}

mod tokenize;

#[cfg(test)]
mod tests {
    
}
