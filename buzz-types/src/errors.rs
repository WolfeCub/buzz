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
}
