use std::collections::HashMap;

/// Holds metadata about the incoming `HttpRequest` that's being handled.
pub struct BuzzContext {
    pub headers: HashMap<String, String>,
}
