use std::error::Error;

use crate::{HttpResponse, HttpStatusCode};

pub trait Respond {
    fn respond(self) -> HttpResponse;
}

impl Respond for () {
    fn respond(self) -> HttpResponse {
        HttpResponse::new(HttpStatusCode::NoContent)
    }
}

impl Respond for String {
    fn respond(self) -> HttpResponse {
        HttpResponse::new(HttpStatusCode::Ok).body(self)
    }
}

impl Respond for &str {
    fn respond(self) -> HttpResponse {
        HttpResponse::new(HttpStatusCode::Ok).body(self.to_owned())
    }
}

impl<T> Respond for Option<T>
where
    T: Respond,
{
    fn respond(self) -> HttpResponse {
        match self {
            Some(thing) => thing.respond(),
            None => HttpResponse::new(HttpStatusCode::NotFound),
        }
    }
}

impl<T, E> Respond for Result<T, E>
where
    T: Respond,
    E: Error,
{
    fn respond(self) -> HttpResponse {
        match self {
            Ok(thing) => thing.respond(),
            Err(e) => HttpResponse::new(HttpStatusCode::InternalServerError).body(e.to_string()),
        }
    }
}
