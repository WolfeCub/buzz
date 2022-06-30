use crate::{HttpMethod, Headers};

/// Representation of an HTTP request that's been parsed.
#[derive(Debug)]
pub struct HttpRequest<'a> {
    pub method: HttpMethod,
    pub path: &'a str,
    pub version: f64,
    pub headers: Headers<'a>,
    pub body: Option<&'a str>,
}

