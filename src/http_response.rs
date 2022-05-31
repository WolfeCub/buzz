use std::collections::HashMap;

pub struct HttpResponse {
    pub status_code: HttpStatusCode,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

impl HttpResponse {
    pub fn new(code: HttpStatusCode) -> Self {
        let mut headers = HashMap::from([
            ("Server".to_owned(), "buzz".to_owned()),
            ("Content-Length".to_owned(), "0".to_owned()),
        ]);

        Self {
            status_code: code,
            headers: headers,
            body: None,
        }
    }

    pub fn body(mut self, body: String) -> Self {
        self.headers.insert("Content-Length".to_owned(), body.len().to_string());
        self.body = Some(body);
        self
    }
}

#[derive(Clone, Copy)]
pub enum HttpStatusCode {
    Ok,
}

impl ToString for HttpStatusCode {
    fn to_string(&self) -> String {
        match self {
            Self::Ok => "OK",
        }
        .to_owned()
    }
}
