use std::collections::HashMap;

use crate::HttpMethod;

#[derive(Debug)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: String,
    pub version: f64,
    pub headers: HashMap<String, String>,
}

