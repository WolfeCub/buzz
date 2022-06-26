use buzz_types::{dev::Parser, errors::JsonParseError};

#[derive(Debug, PartialEq)]
pub(crate) enum JsonTok {
    OpenCurly,
    CloseCurly,
    OpenSquare,
    CloseSquare,
    Comma,
    Colon,
    String(String),
    Number(i64),
    Fractional(f64),
    Bool(bool),
    Null,
}

impl JsonTok {
    pub(crate) fn tokenize<'a>(input: &'a str) -> JsonTokIter<'a> {
        JsonTokIter {
            parser: Parser::new(input.as_bytes()),
        }
    }
}

pub(crate) struct JsonTokIter<'a> {
    parser: Parser<'a>,
}

impl<'a> Iterator for JsonTokIter<'a> {
    type Item = Result<JsonTok, JsonParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.parser.remaining() <= 0 {
                return None;
            }

            return Some(match self.parser.take().unwrap() {
                b' ' => continue,
                b'\t' => continue,
                b'\n' => continue,
                b'\r' => continue,
                b'{' => Ok(JsonTok::OpenCurly),
                b'}' => Ok(JsonTok::CloseCurly),
                b'[' => Ok(JsonTok::OpenSquare),
                b']' => Ok(JsonTok::CloseSquare),
                b',' => Ok(JsonTok::Comma),
                b':' => Ok(JsonTok::Colon),
                b'"' => read_token(&self.parser).map(JsonTok::String),
                b'0'..=b'9' | b'-' => read_num(&self.parser),
                thing => try_read_bool(&self.parser)
                    .or(try_read_null(&self.parser))
                    .ok_or(JsonParseError::UnexpectedToken((thing as char).to_string())),
            });
        }
    }
}

fn try_read_null(parser: &Parser) -> Option<JsonTok> {
    let off = parser.offset();
    if parser.remaining() >= 3 && b"null" == parser.subbytes(off - 1, off + 3) {
        parser.consume(3);
        Some(JsonTok::Null)
    } else {
        None
    }
}

fn try_read_bool(parser: &Parser) -> Option<JsonTok> {
    let off = parser.offset();
    if parser.remaining() >= 3 && b"true" == parser.subbytes(off - 1, off + 3) {
        parser.consume(3);
        Some(JsonTok::Bool(true))
    } else if parser.remaining() >= 4 && b"false" == parser.subbytes(off - 1, off + 4) {
        parser.consume(4);
        Some(JsonTok::Bool(false))
    } else {
        None
    }
}

fn read_num(parser: &Parser) -> Result<JsonTok, JsonParseError> {
    let start = parser.offset() - 1;

    let mut is_fractional = false;
    let mut is_eof = true;
    while let Some(c) = parser.take() {
        if c.is_ascii_digit() {
            continue;
        }

        if c == b'.' {
            if is_fractional {
                return Err(JsonParseError::DuplicateDecimals);
            }
            is_fractional = true;
            continue;
        }

        is_eof = false;
        break;
    }

    if !is_eof {
        parser.rewind(1);
    }

    let candidate = parser.substr_to_offset(start);
    Ok(if is_fractional {
        JsonTok::Fractional(
            candidate
                .parse()
                .map_err(|e| JsonParseError::FractionalParseError(e))?,
        )
    } else {
        JsonTok::Number(
            candidate
                .parse()
                .map_err(|e| JsonParseError::NumberParseError(e))?,
        )
    })
}

fn read_token(parser: &Parser) -> Result<String, JsonParseError> {
    let start = parser.offset() - 1;
    let mut hit_escape = false;
    let mut found_close_quote = false;

    while let Some(c) = parser.take() {
        match c {
            b'\\' if !hit_escape => {
                hit_escape = true;
            }
            b'"' if !hit_escape => {
                found_close_quote = true;
                break;
            }
            _ => {
                hit_escape = false;
            }
        }
    }

    if found_close_quote {
        Ok(parser.substr(start + 1, parser.offset() - 1).to_owned())
    } else {
        Err(JsonParseError::EndOfInputWhile("Parsing string".to_owned()))
    }
}
