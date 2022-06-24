use std::collections::HashMap;

use crate::HttpMethod;

/// Representation of an HTTP request that's been parsed.
#[derive(Debug)]
pub struct HttpRequest<'a> {
    pub method: HttpMethod,
    pub path: &'a str,
    pub version: f64,
    pub headers: HashMap<&'a str, &'a str>,
    pub body: Option<&'a str>,
}

