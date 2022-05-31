use std::{str::FromStr, collections::HashMap};
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

pub fn parse_http(mut lines: impl Iterator<Item = String>) -> Result<HttpRequest, HttpParseError> {
    let first_line = lines.next().ok_or(HttpParseError::Method(
        "No request line provided".to_owned(),
    ))?;

    let (method, rest) = parse_http_method(&first_line)?;
    let (path, rest) = parse_http_path(&rest[1..])?;
    let (version, rest) = parse_http_version(&rest[1..])?;

    let mut headers: HashMap<String, String> = HashMap::new();
    for line in lines {
        let (key, value) = parse_http_header(&line)?;
        headers.insert(key.to_owned(), value.to_owned());
    }

    Ok(dbg!(HttpRequest {
        method,
        path,
        version,
        headers,
    }))
}

fn parse_http_method(line: &str) -> Result<(HttpMethod, &str), HttpParseError> {
    let mut pos = 0;

    for c in line.chars() {
        if c.is_ascii_whitespace() {
            break;
        }
        if !c.is_ascii_uppercase() {
            return Err(HttpParseError::Method(format!(
                "Found non ASCII uppercase character {c} in method"
            )));
        }

        pos += 1;
    }

    if pos == 0 {
        return Err(HttpParseError::Method(
            "Request started with whitespace which may mean no method was specified".to_owned(),
        ));
    }

    let potential_method = &line[0..pos];
    let method = potential_method.parse::<HttpMethod>().map_err(|_| {
        HttpParseError::Method(format!(
            "Provided method {:#?} is not a valid http method",
            potential_method
        ))
    })?;

    Ok((method, &line[pos..]))
}

fn parse_http_path(line: &str) -> Result<(String, &str), HttpParseError> {
    let mut pos = 0;

    for c in line.chars() {
        if c.is_ascii_whitespace() {
            break;
        }

        pos += 1;
    }

    if pos == 0 {
        return Err(HttpParseError::Path(
            "Empty path found requires at least /".to_owned(),
        ));
    }

    Ok((line[..pos].to_owned(), &line[pos..]))
}

fn parse_http_version(line: &str) -> Result<(f64, &str), HttpParseError> {
    let bytes = line.as_bytes();

    if bytes.len() < 5 || &bytes[..5] != b"HTTP/" {
        return Err(HttpParseError::VersionText(
            "Version did not start with 'HTTP/'".to_owned(),
        ));
    }

    let mut pos = 5;
    let mut read_dot = false;

    for char_byte in &bytes[pos..] {
        let c = *char_byte as char;

        if c.is_ascii_whitespace() {
            break;
        }

        if c == '.' && pos > 5 {
            read_dot = true;
            pos += 1;
            continue;
        }

        if !c.is_numeric() {
            return Err(HttpParseError::VersionText(if read_dot {
                "Encountered a second decimal in HTTP version number".to_owned()
            } else {
                format!("Encountered unexpected character {c} in HTTP version number")
            }));
        }

        pos += 1;
    }

    if pos == 5 {
        return Err(HttpParseError::Path("Empty version found".to_owned()));
    }

    unsafe {
        Ok((
            std::str::from_utf8_unchecked(&bytes[5..pos])
                .parse()
                .map_err(HttpParseError::VersionParse)?,
            std::str::from_utf8_unchecked(&bytes[pos..]),
        ))
    }
}

fn parse_http_header(line: &str) -> Result<(&str, &str), HttpParseError> {
    let mut pos = 0;
    let bytes = line.as_bytes();
    let mut read_colon = false;
    for c in bytes.iter() {
        if *c == b':' {
            read_colon = true;
            break;
        }

        if !(*c as char).is_alphabetic() && *c == b'-' && *c == b'_' {
            return Err(HttpParseError::Header(
                format!("Encountered unexpected character: {c} in head").to_string(),
            ));
        }

        pos += 1;
    }

    if pos == 0 {
        return Err(HttpParseError::Header("Header may not be empty".to_owned()));
    }

    if !read_colon {
        return Err(HttpParseError::Header("Header must have a key value delimited by :".to_owned()));
    }

    /* TODO: Not sure how efficient trim is */
    unsafe {
        Ok((
            std::str::from_utf8_unchecked(&bytes[..pos]),
            std::str::from_utf8_unchecked(&bytes[pos + 1..]).trim(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    // #[test]
    // fn debug() {
    //     let result = parse_http_header("0:");
    //     dbg!(&result);

    //     assert!(result.is_err());
    // }

    proptest! {
        #[test]
        fn parses_valid_http_methods(test_method: HttpMethod) {
            let format = format!("{} / HTTP/1.1", test_method.to_string());
            let (method, rest) = parse_http_method(&format).expect("Failed to parse");

            assert_eq!(method, test_method);
            assert_eq!(rest, &format[test_method.to_string().len()..]);
        }

        #[test]
        fn fails_parses_random_http_methods(method: String) {
            let format = format!("{method} / HTTP/1.1");
            let result = parse_http_method(&format);

            assert!(result.is_err());
        }

        #[test]
        fn parses_valid_paths(test_path in "/[A-Za-z0-9-._~!$&'()*+,;=:@%?]+") {
            let format = format!("{test_path} HTTP/1.1");
            let result = parse_http_path(&format);

            assert!(result.is_ok());

            let (path, rest) = result.unwrap();
            assert_eq!(path, test_path);
            assert_eq!(rest, &format[test_path.len()..]);
        }

        #[test]
        fn fails_parses_invalid_paths(test_path in "[A-Z]+[ ]+[A-Z]+") {
            let format = format!("{test_path} HTTP/1.1");
            let result = parse_http_path(&format);

            assert!(result.is_ok());

            let (path, rest) = result.unwrap();
            assert_ne!(path, test_path);
            assert_ne!(rest, &format[test_path.len()..]);
        }

        #[test]
        fn parses_valid_versions(test_version in "[0-9]{1,10}\\.[0-9]{1,10}") {
            let format = format!("HTTP/{test_version}");
            let result = parse_http_version(&format);

            assert!(result.is_ok());

            let (version, rest) = result.unwrap();
            assert_eq!(version, test_version.parse().unwrap());
            assert_eq!(rest, "");
        }

        #[test]
        fn fails_parses_invalid_versions(test_version: String) {
            let result = parse_http_version(&test_version);

            assert!(result.is_err());
        }

        #[test]
        fn parses_valid_headers(test_key in "[A-Za-z-_]+", test_value in r#"[A-Za-z-_:;.,\\/"'?!(){}\[\]@<>=-\\+*#$&`|~\\^%]+"#) {
            let format = format!("{test_key}: {test_value}");
            let result = parse_http_header(&format);

            assert!(result.is_ok());

            let (key, value) = result.unwrap();
            assert_eq!(key, test_key);
            assert_eq!(value, test_value);
        }

        #[test]
        fn fails_parses_invalid_headers(header in "[^:]+") {
            let result = parse_http_header(&header);

            assert!(result.is_err());
        }
    }
}
