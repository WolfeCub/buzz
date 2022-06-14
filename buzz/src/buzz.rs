use std::error::Error;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use crate::http_parse::*;
use crate::routes::*;
use buzz_types::dev::DependancyInjection;
use buzz_types::*;

pub struct Buzz {
    addr: &'static str,
    routes: Routes,
    di: DependancyInjection,
}

impl Buzz {
    pub fn new(addr: &'static str) -> Self {
        Self {
            addr,
            routes: Routes::new(),
            di: DependancyInjection::new(),
        }
    }

    pub fn register<T: 'static>(mut self, injectable: T) -> Self {
        self.di.register(injectable);
        self
    }

    pub fn route(self, route: (Handler, RouteMetadata)) -> Self {
        self.routes(vec![route])
    }

    pub fn routes(mut self, routes: Vec<(Handler, RouteMetadata)>) -> Self {
        self.routes.insert(routes);
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
        let response = self.dispatch(request);

        write_response(&mut stream, &response)?;

        Ok(())
    }

    pub fn dispatch(&self, request: HttpRequest) -> HttpResponse {
        self.routes
            .match_route_params(request, &self.di)
            .unwrap_or_else(|e| {
                HttpResponse::new(HttpStatusCode::InternalServerError).body(e.to_string())
            })
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

    stream.shutdown(std::net::Shutdown::Both)?;
    Ok(())
}
