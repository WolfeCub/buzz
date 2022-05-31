use crate::http_parse::HttpMethod;

#[derive(Debug)]
pub struct HttpService {
    pub path: &'static str,
    pub handler: fn () -> (),
    pub method: HttpMethod,
}
