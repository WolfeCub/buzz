use buzz_types::{
    errors::{DeserializationError, JsonParseError},
    traits::Deserialize,
};

use super::*;

#[derive(Debug, PartialEq, Eq)]
pub enum JsonValue {
    Number(i64),
    Bool(bool),
    String(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
}

impl JsonValue {
    pub fn parse(input: &str) -> Result<JsonValue, JsonParseError> {
        let tokens = JsonTok::tokenize(input);
        parse_expr(&mut tokens.peekable())
    }
}

impl Deserialize<JsonValue> for i64 {
    fn deserialize(val: JsonValue) -> Result<Self, DeserializationError> {
        match val {
            JsonValue::Number(n) => Ok(n),
            thing => Err(DeserializationError::MismatchedTypes(
                "Number".to_owned(),
                thing.to_string(),
            )),
        }
    }
}

impl Deserialize<JsonValue> for bool {
    fn deserialize(val: JsonValue) -> Result<Self, DeserializationError> {
        match val {
            JsonValue::Bool(b) => Ok(b),
            thing => Err(DeserializationError::MismatchedTypes(
                "Bool".to_owned(),
                thing.to_string(),
            )),
        }
    }
}

impl Deserialize<JsonValue> for String {
    fn deserialize(val: JsonValue) -> Result<Self, DeserializationError> {
        match val {
            JsonValue::String(s) => Ok(s),
            thing => Err(DeserializationError::MismatchedTypes(
                "String".to_owned(),
                thing.to_string(),
            )),
        }
    }
}

impl<T: Deserialize<JsonValue>> Deserialize<JsonValue> for Vec<T> {
    fn deserialize(val: JsonValue) -> Result<Self, DeserializationError> {
        match val {
            JsonValue::Array(a) => Ok(a
                .into_iter()
                .map(|x| T::deserialize(x))
                .collect::<Result<Vec<_>, _>>()?),
            thing => Err(DeserializationError::MismatchedTypes(
                "Array".to_owned(),
                thing.to_string(),
            )),
        }
    }
}

