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
        self.headers.push(("Content-Length".to_owned(), body.len().to_string()));
        self.body = Some(body);
        self
    }

}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum HttpStatusCode {
    Ok = 200,
    NoContent = 204,
    NotFound = 404,
    InternalServerError = 500,
}

impl ToString for HttpStatusCode {
    fn to_string(&self) -> String {
        match self {
            Self::Ok => "OK",
            Self::NoContent => "No Content",
            Self::NotFound => "Not Found",
            Self::InternalServerError => "Internal Server Error",
        }
        .to_owned()
    }
}
