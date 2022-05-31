use std::{collections::HashMap, str::FromStr};
use thiserror::Error;

#[cfg(test)]
use proptest_derive::Arbitrary;

#[derive(Error, Debug)]
pub enum HttpParseError {
    #[error("HttpParseError Method: `{0}`")]
    Method(String),

    #[error("HttpParseError Path: `{0}`")]
    Path(String),

    #[error("HttpParseError Version: `{0}`")]
    VersionText(String),

    #[error("HttpParseError Version: `{0}`")]
    VersionParse(#[from] std::num::ParseFloatError),

    #[error("HttpParseError Header: `{0}`")]
    Header(String),
}

#[cfg_attr(test, derive(Arbitrary))]
#[derive(Debug, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Put,
    Post,
    Delete,
    Patch,
    Options,
}

impl ToString for HttpMethod {
    fn to_string(&self) -> String {
        match self {
            Self::Get => "GET",
            Self::Put => "PUT",
            Self::Post => "POST",
            Self::Delete => "DELETE",
            Self::Patch => "PATCH",
            Self::Options => "OPTIONS",
        }
        .to_string()
    }
}

impl FromStr for HttpMethod {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(HttpMethod::Get),
            "PUT" => Ok(HttpMethod::Put),
            "POST" => Ok(HttpMethod::Post),
            "DELETE" => Ok(HttpMethod::Delete),
            "PATCH" => Ok(HttpMethod::Patch),
            "OPTIONS" => Ok(HttpMethod::Options),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: String,
    pub version: f64,
    pub headers: HashMap<String, String>,
}

#[derive(Debug)]
struct Parser<'a> {
    data: &'a [u8],
    offset: usize,
}

impl<'a> Parser<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data, offset: 0 }
    }

    fn peek(&self) -> Option<u8> {
        if self.offset >= self.data.len() {
            None
        } else {
            let r = self.data[self.offset];
            Some(r)
        }
    }

    fn consume(&mut self, n: usize) {
        self.offset += n;
    }

    fn consume_while(&mut self, predicate: fn(u8) -> bool) {
        while let Some(_) = self.take_if(predicate) {}
    }

    fn take(&mut self) -> Option<u8> {
        let r = self.peek();
        if r.is_some() {
            self.offset += 1;
        }
        r
    }

    fn take_n(&mut self, n: usize) -> Option<&[u8]> {
        if self.offset + n <= self.data.len() {
            let r = Some(&self.data[self.offset..self.offset + n]);
            self.offset += n;
            r
        } else {
            None
        }
    }

    fn take_if(&mut self, predicate: fn(u8) -> bool) -> Option<u8> {
        let r = self.peek();
        if r.is_some() && r.map(predicate).unwrap() {
            self.offset += 1;
            r
        } else {
            None
        }
    }

    fn substr_to_offset(&self, starting: usize) -> &str {
        self.substr(starting, self.offset)
    }

    fn substr(&self, starting: usize, ending: usize) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.data[starting..ending]) }
    }
}

pub fn parse_http(request: &[u8]) -> Result<HttpRequest, HttpParseError> {
    let mut parser = Parser::new(request);

    let method = parse_http_method(&mut parser)?;
    let path = parse_http_path(&mut parser)?;
    let version = parse_http_version(&mut parser)?;

    let mut headers: HashMap<String, String> = HashMap::new();
    while let Some((key, val)) = parse_http_header(&mut parser)? {
        headers.insert(key.to_owned(), val.to_owned());
    }

    Ok(dbg!(HttpRequest {
        method,
        path,
        version,
        headers,
    }))
}

fn parse_http_method<'a>(parser: &mut Parser<'a>) -> Result<HttpMethod, HttpParseError> {
    let starting_pos = parser.offset;

    while let Some(c) = parser.take_if(|c| c != b'\r') {
        if c.is_ascii_whitespace() {
            break;
        }
        if !c.is_ascii_uppercase() {
            return Err(HttpParseError::Method(format!(
                "Found non ASCII uppercase character {c} in method"
            )));
        }
    }

    if starting_pos == parser.offset {
        return Err(HttpParseError::Method(
            "Request started with whitespace which may mean no method was specified".to_owned(),
        ));
    }

    let potential_method = parser.substr(starting_pos, parser.offset - 1);
    let method = potential_method.parse::<HttpMethod>().map_err(|_| {
        HttpParseError::Method(format!(
            "Provided method {:#?} is not a valid http method",
            potential_method
        ))
    })?;

    Ok(method)
}

fn parse_http_path<'a>(parser: &mut Parser<'a>) -> Result<String, HttpParseError> {
    let starting_pos = parser.offset;

    while let Some(c) = parser.take() {
        if c.is_ascii_whitespace() {
            break;
        }
    }

    if starting_pos == parser.offset {
        return Err(HttpParseError::Path(
            "Empty path found requires at least /".to_owned(),
        ));
    }

    Ok(parser.substr(starting_pos, parser.offset - 1).to_owned())
}

fn parse_http_version<'a>(parser: &mut Parser<'a>) -> Result<f64, HttpParseError> {
    let starting_pos = parser.offset;

    for i in 0..5 {
        match parser.take() {
            Some(_) => {}
            None => {
                return Err(HttpParseError::VersionText(
                    "Unexpected end of input mid HTTP version".to_owned(),
                ));
            }
        }
    }

    if parser.substr_to_offset(starting_pos) != "HTTP/" {
        return Err(HttpParseError::VersionText(
            "Version did not start with 'HTTP/'".to_owned(),
        ));
    }

    let start_of_version = parser.offset;

    parser.consume_while(|c| c != b'\r');

    dbg!(
        std::str::from_utf8(&parser.data[parser.offset..parser.offset + 2]),
        parser.offset
    );

    if !eat_newline(parser) {
        return Err(HttpParseError::Path(
            "Expected network newline after version number".to_owned(),
        ));
    }

    if start_of_version == parser.offset {
        return Err(HttpParseError::Path("Empty version found".to_owned()));
    }

    Ok(parser
        .substr(start_of_version, parser.offset - 2)
        .parse()
        .map_err(HttpParseError::VersionParse)?)
}

fn parse_http_header<'a>(
    parser: &mut Parser<'a>,
) -> Result<Option<(String, String)>, HttpParseError> {
    let starting_pos = parser.offset;
    let mut found_colon = false;

    while let Some(c) = parser.take() {
        if c == b':' {
            found_colon = true;
            break;
        }

        if c == b'\r' {
            break;
        }

        if !c.is_ascii_alphabetic() && c != b'-' && c != b'_' {
            return Err(HttpParseError::Header(
                format!("Encountered unexpected character: {c} in head").to_string(),
            ));
        }
    }

    if starting_pos == parser.offset {
        return Err(HttpParseError::Header("Header may not be empty".to_owned()));
    }

    if let Some(c) = parser.peek() {
        if c == b'\n' {
            return Ok(None);
        }
    }

    if found_colon == false {
        return Err(HttpParseError::Header(
            "Header must use : to delimit key and value".to_owned(),
        ));
    }

    let key = parser.substr(starting_pos, parser.offset - 1).to_owned();
    let value_pos = parser.offset;

    parser.consume_while(|c| c != b'\r');

    if !eat_newline(parser) {
        return Err(HttpParseError::Header(
            "Expected header to end with network newline".to_owned(),
        ));
    }

    /* TODO: Not sure how efficient trim is */
    Ok(Some((
        key,
        parser
            .substr(value_pos, parser.offset - 1)
            .trim()
            .to_owned(),
    )))
}

fn eat_newline<'a>(parser: &mut Parser<'a>) -> bool {
    let network_newline = parser.take_n(2);

    return network_newline.is_some() && network_newline.unwrap() == b"\r\n";
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn debug() {
        let format = "\r\n";
        let mut parser = Parser::new(format.as_bytes());

        dbg!(parse_http_header(&mut parser));
    }

    proptest! {
        #[test]
        fn parses_valid_http_methods(test_method: HttpMethod) {
            let format = format!("{} / HTTP/1.1", test_method.to_string());

            let mut parser = Parser::new(format.as_bytes());
            let result = parse_http_method(&mut parser);

            assert!(result.is_ok());

            assert_eq!(result.unwrap(), test_method);
            assert_eq!(parser.offset - 1, test_method.to_string().len());
        }

        #[test]
        fn fails_parses_random_http_methods(method: String) {
            let format = format!("{method} / HTTP/1.1");
            let mut parser = Parser::new(format.as_bytes());
            let result = parse_http_method(&mut parser);

            assert!(result.is_err());
        }

        #[test]
        fn parses_valid_paths(test_path in "/[A-Za-z0-9-._~!$&'()*+,;=:@%?]+") {
            let format = format!("{test_path} HTTP/1.1");

            let mut parser = Parser::new(format.as_bytes());
            let result = parse_http_path(&mut parser);

            assert!(result.is_ok());

            assert_eq!(result.unwrap(), test_path);
            assert_eq!(parser.offset - 1, test_path.len());
        }

        #[test]
        fn fails_parses_invalid_paths(test_path in "[A-Z]+[ ]+[A-Z]+") {
            let format = format!("{test_path} HTTP/1.1");

            let mut parser = Parser::new(format.as_bytes());
            let result = parse_http_path(&mut parser);

            assert!(result.is_ok());

            assert_ne!(result.unwrap(), test_path);
            assert_ne!(parser.offset - 1, test_path.len());
        }

        #[test]
        fn parses_valid_versions(test_version in "[0-9]{1,10}\\.[0-9]{1,10}") {
            let format = format!("HTTP/{test_version}\r\n");

            let mut parser = Parser::new(format.as_bytes());
            let result = parse_http_version(&mut parser);

            assert!(result.is_ok());

            let version = result.unwrap();
            assert_eq!(version, test_version.parse().unwrap());
            assert_eq!(parser.offset, parser.data.len());
        }

        #[test]
        fn fails_parses_invalid_versions(test_version: String) {
            let mut parser = Parser::new(test_version.as_bytes());
            let result = parse_http_version(&mut parser);

            assert!(result.is_err());
        }

        #[test]
        fn parses_valid_headers(test_key in "[A-Za-z-_]+", test_value in r#"[A-Za-z-_:;.,\\/"'?!(){}\[\]@<>=-\\+*#$&`|~\\^%]+"#) {
            let format = format!("{test_key}: {test_value}\r\n");

            let mut parser = Parser::new(format.as_bytes());
            let result = parse_http_header(&mut parser);

            assert!(result.is_ok());

            let option = result.unwrap();

            assert!(option.is_some());

            let (key, value) = option.unwrap();
            assert_eq!(key, test_key);
            assert_eq!(value, test_value);
        }

        #[test]
        fn fails_parses_invalid_headers(header in "[^:^\r^\n]+") {
            let mut parser = Parser::new(header.as_bytes());
            let result = parse_http_header(&mut parser);

            assert!(result.is_err());
        }
    }
}
