use std::str::FromStr;

use thiserror::Error;


#[derive(Error, Debug)]
pub enum HttpParseError {
    #[error("HttpParseError Method: `{0}`")]
    Method(String),
    #[error("HttpParseError Path: `{0}`")]
    Path(String),
    #[error("HttpParseError Version: `{0}`")]
    VersionText(String),
    #[error("HttpParseError Version: `{0}`")]
    VersionParse(#[from] std::num::ParseFloatError),
    #[error("HttpParseError Header: `{0}`")]
    Header(String),
    #[error("HttpParseError: Missing newline after headers")]
    MissingNewlineAfterHeaders,
}


#[derive(Error, Debug)]
pub enum RouteParseError {
    #[error("RotueParseError: Missing leading slash on route")]
    MissingLeadingSlash,
}


#[derive(Error, Debug)]
pub enum BuzzError {
    #[error("Use of unregistered type: `{0}`")]
    UseOfUnregesteredInject(String),
    #[error("{0}")]
    BadRequest(String),
    #[error("Lock for injected type `{0}` was poisoned on `{1}`")]
    LockAcquirePoisoned(String, String),
    #[error("Body parsing error: {0}")]
    BodyParseError(String),
}

#[derive(Error, Debug)]
pub enum JsonError {
    #[error("{0}")]
    ParseError(#[from] JsonParseError),

    #[error("{0}")]
    DeserializationError(#[from] DeserializationError),
}

#[derive(Error, Debug)]
pub enum JsonParseError {
    #[error("Unexpected token '{0}")]
    UnexpectedToken(String),
    #[error("End of input while {0}")]
    EndOfInputWhile(String),
    #[error("Expected comma but none was found")]
    ExpectedComma,
    #[error("Expected colon but none was found")]
    ExpectedColon,
    #[error("Invalid object key found '{0}' expected string")]
    InvalidObjectKey(String),
    #[error("Numbers may only contain one '.'")]
    DuplicateDecimals,
    #[error("{0}")]
    NumberParseError(#[source] <i64 as FromStr>::Err),
    #[error("{0}")]
    FractionalParseError(#[source] <f64 as FromStr>::Err),
}

#[derive(Error, Debug)]
pub enum DeserializationError {
    #[error("Mismatch types. Expected `{0}` got `{1}` ")]
    MismatchedTypes(String, String),
    #[error("Expected {0} keys but found {1}")]
    MissingValues(usize, usize),
}
