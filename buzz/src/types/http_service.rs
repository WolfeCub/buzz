#[derive(Debug, Clone, Copy)]
pub struct HttpService {
    pub path: &'static str,
    pub handler: fn () -> String,
    pub method: &'static str, /* TODO: Enumify this */
}
