use std::collections::HashMap;

mod tokenizer;
use self::tokenizer::*;


#[cfg(test)]
mod tests;

#[derive(Debug)]
enum JsonValue {
    Number(i64),
    Bool(bool),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

pub (crate) fn parse_json(input: &str) {
    let tokens = tokenize(input);
}

