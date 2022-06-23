use std::collections::HashMap;

use crate::HttpMethod;

/// Representation of an HTTP request that's been parsed.
#[derive(Debug)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: String,
    pub version: f64,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

