use std::error::Error;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use crate::http_parse::*;
use buzz_types::*;

/* TODO: Use enum in the handler map rather than strings */
pub struct Buzz {
    addr: &'static str,
    handlers: Vec<fn(&HttpRequest) -> Option<HttpResponse>>,
}

impl Buzz {
    pub fn new(addr: &'static str) -> Self {
        Self {
            addr,
            handlers: Vec::new(),
        }
    }

    pub fn route(mut self, handler: fn(&HttpRequest) -> Option<HttpResponse>) -> Self {
        self.handlers.push(handler);
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

        for handler in self.handlers.iter() {
            if let Some(response) = (*handler)(&request) {
                write_response(&mut stream, &response)?;

                stream.flush()?;
                stream.shutdown(std::net::Shutdown::Both)?;
                return Ok(());
            }
        }

        let response = HttpResponse::new(HttpStatusCode::NotFound);

        write_response(&mut stream, &response)?;

        stream.flush()?;
        stream.shutdown(std::net::Shutdown::Both)?;

        Ok(())
    }
}

fn write_response(stream: &mut TcpStream, request: &HttpResponse) -> std::io::Result<()> {
    /* TODO: Not hardcoded version. What do we actually support? */
    stream.write(b"HTTP/1.1 ")?;
    stream.write((request.status_code as u32).to_string().as_bytes())?;
    stream.write(b" ")?;
    stream.write(request.status_code.to_string().as_bytes())?;
    stream.write(b"\r\n")?;

    for (key, value) in &request.headers {
        stream.write(key.as_bytes())?;
        stream.write(b": ")?;
        stream.write(value.as_bytes())?;
        stream.write(b"\r\n")?;
    }

    stream.write(b"\r\n")?;

    /* TODO: Buffer? */
    if let Some(body) = &request.body {
        stream.write(body.as_bytes())?;
    }
    stream.flush()?;

    Ok(())
}
