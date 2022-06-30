use crate::Headers;

/// Holds metadata about the incoming `HttpRequest` that's being handled.
pub struct BuzzContext<'a> {
    pub headers: Headers<'a>,
}
