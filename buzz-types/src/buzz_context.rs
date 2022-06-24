use std::collections::HashMap;

/// Holds metadata about the incoming `HttpRequest` that's being handled.
pub struct BuzzContext<'a> {
    pub headers: HashMap<&'a str, &'a str>,
}
