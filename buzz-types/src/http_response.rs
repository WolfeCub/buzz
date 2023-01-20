/// Holds the data that will be formatted back into an HTTP response by the server.
///
/// This type is what [`respond`](crate::traits::Respond::respond) returns and since all
/// [`Handler`](crate::handler::Handler) functions return a [`Respond`](crate::traits::Respond::respond)
/// trait we can map from a `T where T: Respond` to a `HttpResponse`.
///
/// # Example
///
/// Consider the simple [`Handler`](crate::handler::Handler):
/// ```ignore
/// #[get("/foo")]
/// fn foo() -> impl Respond {
///     "foo"
/// }
/// ```
/// This function returns the concrete type `&str` and since [`Respond`](crate::traits::Respond) is
/// implemented for `&str` we can generate an `HttpResponse` to be sent to the caller.
#[derive(Debug)]
pub struct HttpResponse {
    pub status_code: HttpStatusCode,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

impl HttpResponse {
    pub fn new(code: HttpStatusCode) -> Self {
        let headers = Vec::from([
            ("Server".to_owned(), "buzz".to_owned()),
            ("Content-Length".to_owned(), "0".to_owned()),
        ]);

        Self {
            status_code: code,
            headers,
            body: None,
        }
    }

    pub fn status(mut self, code: HttpStatusCode) -> Self {
        self.status_code = code;
        self
    }

    pub fn body(mut self, body: String) -> Self {
        self.headers
            .push(("Content-Length".to_owned(), body.len().to_string()));
        self.body = Some(body);
        self
    }
}

/// Represents the status code of an HTTP response.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum HttpStatusCode {
    Ok = 200,
    NoContent = 204,
    BadRequest = 400,
    NotFound = 404,
    ImATeapot = 418,
    InternalServerError = 500,
}

impl ToString for HttpStatusCode {
    fn to_string(&self) -> String {
        match self {
            Self::Ok => "OK",
            Self::NoContent => "No Content",
            Self::BadRequest => "Bad Request",
            Self::NotFound => "Not Found",
            Self::ImATeapot => "I'm a teapot",
            Self::InternalServerError => "Internal Server Error",
        }
        .to_owned()
    }
}
