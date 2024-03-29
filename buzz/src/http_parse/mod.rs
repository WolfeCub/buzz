use buzz_types::dev::*;
use buzz_types::errors::HttpParseError;
use buzz_types::*;

#[cfg(test)]
mod tests;

pub fn parse_http(request: &[u8]) -> Result<HttpRequest, HttpParseError> {
    let parser = Parser::new(request);

    let method = parse_http_method(&parser)?;
    let path = parse_http_path(&parser)?;
    let version = parse_http_version(&parser)?;

    let kvps = parse_http_headers(&parser)?;
    let headers = Headers::from_iter(kvps);


    if Some(b'\n') != parser.take() {
        return Err(HttpParseError::MissingNewlineAfterHeaders);
    }

    let body = headers.content_length.and_then(|val| {
        if let Ok(num) = val.parse::<usize>() {
            Some(parser.substr(parser.offset(), parser.offset() + num))
        } else {
            None
        }
    });

    Ok(HttpRequest {
        method,
        path,
        version,
        headers,
        body,
    })
}

fn parse_http_method<'a>(parser: &Parser<'a>) -> Result<HttpMethod, HttpParseError> {
    let starting_pos = parser.offset();

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

    if starting_pos == parser.offset() {
        return Err(HttpParseError::Method(
            "Request started with whitespace which may mean no method was specified".to_owned(),
        ));
    }

    let potential_method = parser.substr(starting_pos, parser.offset() - 1);
    let method = potential_method.parse::<HttpMethod>().map_err(|_| {
        HttpParseError::Method(format!(
            "Provided method {:#?} is not a valid http method",
            potential_method
        ))
    })?;

    Ok(method)
}

fn parse_http_path<'a>(parser: &Parser<'a>) -> Result<&'a str, HttpParseError> {
    let starting_pos = parser.offset();

    while let Some(c) = parser.take() {
        if c.is_ascii_whitespace() {
            break;
        }
    }

    if starting_pos == parser.offset() {
        return Err(HttpParseError::Path(
            "Empty path found requires at least /".to_owned(),
        ));
    }

    Ok(parser.substr(starting_pos, parser.offset() - 1))
}

fn parse_http_version<'a>(parser: &Parser<'a>) -> Result<f64, HttpParseError> {
    let starting_pos = parser.offset();

    for _ in 0..5 {
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

    let start_of_version = parser.offset();

    parser.consume_while(|c| c != b'\r');

    if !eat_newline(parser) {
        return Err(HttpParseError::Path(
            "Expected network newline after version number".to_owned(),
        ));
    }

    if start_of_version == parser.offset() {
        return Err(HttpParseError::Path("Empty version found".to_owned()));
    }

    Ok(parser
        .substr(start_of_version, parser.offset() - 2)
        .parse()
        .map_err(HttpParseError::VersionParse)?)
}

fn parse_http_headers<'a>(parser: &Parser<'a>) -> Result<Vec<(&'a str, &'a str)>, HttpParseError> {
    let mut vec = Vec::new();
    while let Some(kvp) = parse_http_header(&parser)? {
        vec.push(kvp);
    };
    Ok(vec)
}

fn parse_http_header<'a>(
    parser: &Parser<'a>,
) -> Result<Option<(&'a str, &'a str)>, HttpParseError> {
    let starting_pos = parser.offset();
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

    if starting_pos == parser.offset() {
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

    let key = parser.substr(starting_pos, parser.offset() - 1);
    let value_pos = parser.offset();

    parser.consume_while(|c| c != b'\r');

    if !eat_newline(parser) {
        return Err(HttpParseError::Header(
            "Expected header to end with network newline".to_owned(),
        ));
    }

    /* TODO: Not sure how efficient trim is */
    Ok(Some((
        key,
        parser.substr(value_pos, parser.offset() - 1).trim(),
    )))
}

fn eat_newline<'a>(parser: &Parser<'a>) -> bool {
    let network_newline = parser.take_n(2);

    return network_newline.is_some() && network_newline.unwrap() == b"\r\n";
}
