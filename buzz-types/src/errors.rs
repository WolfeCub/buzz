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
    #[error("Json parsing error: {0}")]
    JsonParseError(#[from] Box<dyn std::error::Error>),
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
}
