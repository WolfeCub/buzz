use crate::types::{HttpResponse, HttpStatusCode};

pub trait Respond {
    fn respond(self) -> HttpResponse;
}

impl Respond for String {
    fn respond(self) -> HttpResponse {
        HttpResponse::new(HttpStatusCode::Ok)
            .body(self)
    }
}

impl Respond for &'static str {
    fn respond(self) -> HttpResponse {
        HttpResponse::new(HttpStatusCode::Ok)
            .body(self.to_owned())
    }
}
