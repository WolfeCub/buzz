use std::collections::HashMap;

#[derive(Debug)]
pub struct HttpResponse {
    pub status_code: HttpStatusCode,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

impl HttpResponse {
    pub fn new(code: HttpStatusCode) -> Self {
        let headers = HashMap::from([
            ("Server".to_owned(), "buzz".to_owned()),
            ("Content-Length".to_owned(), "0".to_owned()),
        ]);

        Self {
            status_code: code,
            headers,
            body: None,
        }
    }

    pub fn body(mut self, body: String) -> Self {
        self.headers.insert("Content-Length".to_owned(), body.len().to_string());
        self.body = Some(body);
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub enum HttpStatusCode {
    Ok,
    NotFound,
}

impl ToString for HttpStatusCode {
    fn to_string(&self) -> String {
        match self {
            Self::Ok => "OK",
            Self::NotFound => "Not Found",
        }
        .to_owned()
    }
}

pub fn to_status_num(e: HttpStatusCode) -> u32 {
    match e {
        HttpStatusCode::Ok => 200,
        HttpStatusCode::NotFound => 404,
    }
}
