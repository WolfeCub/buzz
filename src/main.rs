use std::collections::HashMap;
use std::error::Error;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpListener;
use std::net::TcpStream;

mod http_parse;
use http_parse::*;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        match handle_connection(stream) {
            Ok(_) => {}
            Err(e) => panic!("{}", e),
        }
    }
}

fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer)?;

    let request = parse_http(&buffer)?;

    let resp_data: &[u8] = "<h1>Hello there</h1>".as_bytes();

    let mut headers = HashMap::new();

    headers.insert("Server".to_owned(), "buzz".to_owned());
    headers.insert("Content-Length".to_owned(), resp_data.len().to_string());

    let response = HttpResponse {
        status_code: HttpStatusCode::Ok,
        headers: headers,
        body: resp_data,
        ..Default::default()
    };

    write_response(&mut stream, response)?;

    stream.flush()?;
    stream.shutdown(std::net::Shutdown::Both)?;

    Ok(())
}

struct HttpResponse<'a> {
    status_code: HttpStatusCode,
    headers: HashMap<String, String>,
    body: &'a [u8],
}

impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        Self {
            status_code: HttpStatusCode::Ok,
            headers: Default::default(),
            body: Default::default(),
        }
    }
}

#[derive(Clone, Copy)]
enum HttpStatusCode {
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

fn to_status_num(e: HttpStatusCode) -> u32 {
    match e {
        HttpStatusCode::Ok => 200,
    }
}

fn write_response(stream: &mut TcpStream, request: HttpResponse) -> std::io::Result<()> {
    /* TODO: Not hardcoded version. What do we actually support? */
    stream.write(
        format!(
            "HTTP/1.1 {} {}\r\n",
            to_status_num(request.status_code),
            request.status_code.to_string()
        )
        .as_bytes(),
    )?;

    for (key, value) in request.headers {
        stream.write(format!("{}: {}\r\n", key, value).as_bytes())?;
    }

    stream.write(b"\r\n")?;

    /* TODO: Buffer? */
    stream.write(request.body)?;

    Ok(())
}
