use std::collections::HashMap;
use std::error::Error;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use crate::http_parse::*;
use buzz_types::*;

/* TODO: Use enum in the handler map rather than strings */
pub struct Buzz {
    addr: &'static str,
    handlers: HashMap<RouteMetadata<'static>, fn() -> HttpResponse>,
}

impl Buzz {
    pub fn new(addr: &'static str) -> Self {
        Self {
            addr,
            handlers: HashMap::new(),
        }
    }

    pub fn route(mut self, route: (fn() -> HttpResponse, RouteMetadata<'static>)) -> Self {
        self.handlers.insert(route.1, route.0);
        self
    }

    pub fn run_server(&self) {
        let listener = TcpListener::bind(self.addr).unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            match self.handle_connection(stream) {
                Ok(_) => {}
                Err(e) => panic!("{}", e),
            }
        }
    }

    fn handle_connection(&self, mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
        let mut buffer = [0; 1024];

        stream.read(&mut buffer)?;

        let request = parse_http(&buffer)?;

        let metadata = RouteMetadata {
            method: &request.method.to_string(),
            path: &request.path,
        };
        let response = match self.handlers.get(&metadata) {
            Some(handler) => handler(),
            None => HttpResponse::new(HttpStatusCode::NotFound),
        };

        write_response(&mut stream, &response)?;

        stream.flush()?;
        stream.shutdown(std::net::Shutdown::Both)?;

        Ok(())
    }
}

fn write_response(stream: &mut TcpStream, request: &HttpResponse) -> std::io::Result<()> {
    /* TODO: Not hardcoded version. What do we actually support? */
    stream.write(
        format!(
            "HTTP/1.1 {} {}\r\n",
            to_status_num(request.status_code),
            request.status_code.to_string()
        )
        .as_bytes(),
    )?;

    for (key, value) in &request.headers {
        stream.write(format!("{}: {}\r\n", key, value).as_bytes())?;
    }

    stream.write(b"\r\n")?;

    /* TODO: Buffer? */
    if let Some(body) = &request.body {
        stream.write(body.as_bytes())?;
    }
    stream.flush()?;

    Ok(())
}
