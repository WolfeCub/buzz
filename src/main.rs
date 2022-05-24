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

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(stream);

    match parse_http(buf_reader.lines().map(|l| l.unwrap())) {
        Ok(_) => {}
        Err(e) => panic!("{}", e),
    }
}

