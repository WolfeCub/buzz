use std::iter::Peekable;

mod tokenizer;
use buzz_types::{errors::JsonParseError, traits::FromBody};

use self::tokenizer::*;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq, Eq)]
pub enum Json {
    Number(i64),
    Bool(bool),
    String(String),
    Array(Vec<Json>),
    Object(Vec<(String, Json)>),
}

impl Json {
    pub fn parse(input: &str) -> Result<Json, JsonParseError> {
        let tokens = JsonTok::tokenize(input);
        parse_expr(&mut tokens.peekable())
    }
}

impl ToString for Json {
    fn to_string(&self) -> String {
        match self {
            Json::Number(num) => num.to_string(),
            Json::Bool(boolean) => boolean.to_string(),
            Json::String(string) => format!(r#""{}""#, string),
            Json::Array(arr) => format!(
                "[{}]",
                arr.iter()
                    .map(|elem| elem.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Json::Object(arr) => format!(
                "{{{}}}",
                arr.iter()
                    .map(|(k, v)| format!(r#""{}": {}"#, k, v.to_string()))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        }
    }
}

impl FromBody for Json {
    fn from_body(body: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Json::parse(body)?)
    }
}

fn parse_expr(tokens: &mut Peekable<JsonTokIter>) -> Result<Json, JsonParseError> {
    fn make_err<T>(string: &str) -> Result<T, JsonParseError> {
        Err(JsonParseError::UnexpectedToken(string.to_owned()))
    }

    match tokens.next().unwrap()? {
        JsonTok::Comma => make_err(","),
        JsonTok::Colon => make_err(":"),
        JsonTok::CloseSquare => make_err("]"),
        JsonTok::CloseCurly => make_err("}"),

        JsonTok::String(s) => Ok(Json::String(s)),
        JsonTok::Number(n) => Ok(Json::Number(n)),
        JsonTok::Bool(b) => Ok(Json::Bool(b)),
        JsonTok::OpenSquare => parse_array(tokens),
        JsonTok::OpenCurly => parse_object(tokens),
    }
}

fn parse_array(tokens: &mut Peekable<JsonTokIter>) -> Result<Json, JsonParseError> {
    let mut result = Vec::new();
    let mut comma_expected = false;

    loop {
        let next = tokens.peek();

        if next.is_none() {
            return Err(JsonParseError::EndOfInputWhile(
                "parsing array elements".to_owned(),
            ));
        }

        /* TODO: Less gross */
        let value = if next.unwrap().is_err() {
            let failed = tokens.next().unwrap();
            return Err(failed.unwrap_err());
        } else {
            next.unwrap().as_ref().unwrap()
        };

        if *value == JsonTok::CloseSquare {
            tokens.next();
            break;
        }

        if comma_expected {
            if *value != JsonTok::Comma {
                return Err(JsonParseError::ExpectedComma);
            } else {
                tokens.next();
                comma_expected = false;
                continue;
            }
        }

        let elem = parse_expr(tokens)?;
        result.push(elem);
        comma_expected = true;
    }

    Ok(Json::Array(result))
}

fn parse_object(tokens: &mut Peekable<JsonTokIter>) -> Result<Json, JsonParseError> {
    let mut result = Vec::new();
    let mut comma_expected = false;

    loop {
        let next = tokens.next();

        if next.is_none() {
            return Err(JsonParseError::EndOfInputWhile(
                "parsing object values".to_owned(),
            ));
        }

        let value = next.unwrap()?;

        if value == JsonTok::CloseCurly {
            break;
        }

        if comma_expected {
            if value == JsonTok::Comma {
                comma_expected = false;
                continue;
            } else {
                return Err(JsonParseError::ExpectedComma);
            }
        }

        /* TODO: Unnest this */
        if let JsonTok::String(key) = value {
            if let Some(Ok(JsonTok::Colon)) = tokens.next() {
                let thing = parse_expr(tokens)?;
                result.push((key, thing));
                comma_expected = true;
            } else {
                return Err(JsonParseError::ExpectedColon);
            }
        } else {
            return Err(JsonParseError::InvalidObjectKey(format!("{:#?}", value)));
        }
    }

    Ok(Json::Object(result))
}
