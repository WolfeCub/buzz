use std::error::Error;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::net::TcpListener;
use std::net::TcpStream;

use crate::http_parse::*;
use crate::routes::*;
use buzz_types::dev::DependancyInjection;
use buzz_types::errors::BuzzError;
use buzz_types::*;

pub struct Buzz {
    addr: &'static str,
    routes: Routes,
    di: DependancyInjection,
    middleware: Vec<Middleware>,
}

impl Buzz {
    pub fn new(addr: &'static str) -> Self {
        Self {
            addr,
            routes: Routes::new(),
            di: DependancyInjection::new(),
            middleware: vec![],
        }
    }

    pub fn routes(mut self, routes: Vec<(Handler, RouteMetadata)>) -> Self {
        self.routes.insert(routes);
        self
    }

    pub fn router(self, fun: fn(Self) -> Self) -> Self {
        fun(self)
    }

    pub fn register<T: 'static>(mut self, injectable: T) -> Self {
        self.di.register(injectable);
        self
    }

    pub fn middleware(mut self, middleware: Middleware) -> Self {
        self.middleware.push(middleware);
        self
    }

    pub fn run_server(&self) {
        let listener = TcpListener::bind(self.addr)
            .expect(format!("Unabled to bind to: {}", self.addr).as_str());

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            if let Err(e) = self.handle_connection(stream) {
                eprintln!("{}", e);
            }
        }
    }

    fn handle_connection(&self, stream: TcpStream) -> Result<(), Box<dyn Error>> {
        let mut buf_reader = BufReader::new(stream);

        let mut buffer = [0; 1024];

        buf_reader.read(&mut buffer)?;

        let request = parse_http(&buffer)?;
        let response = self.dispatch(request);

        let mut buf_writer = BufWriter::new(buf_reader.into_inner());

        write_response(&mut buf_writer, &response)?;

        /* Into inner flushes for us */
        buf_writer
            .into_inner()?
            .shutdown(std::net::Shutdown::Both)?;

        Ok(())
    }

    pub fn dispatch(&self, request: HttpRequest) -> HttpResponse {
        let mut r = request;
        for middleware in self.middleware.iter() {
            match middleware(r) {
                Ok(req) => r = req,
                Err(resp) => return resp,
            }
        }
        match self.routes.match_route_params(r, &self.di) {
            Ok(response) => response,
            Err(BuzzError::BadRequest(err)) => {
                HttpResponse::new(HttpStatusCode::BadRequest).body(err.to_string())
            }
            Err(err) => {
                HttpResponse::new(HttpStatusCode::InternalServerError).body(err.to_string())
            }
        }
    }
}

fn write_response<W: Write>(stream: &mut W, request: &HttpResponse) -> std::io::Result<()> {
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

    if let Some(body) = &request.body {
        stream.write(body.as_bytes())?;
    }
    Ok(())
}
