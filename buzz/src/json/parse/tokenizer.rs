use buzz_types::{dev::Parser, errors::JsonParseError};

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum JsonTok {
    OpenCurly,
    CloseCurly,
    OpenSquare,
    CloseSquare,
    Comma,
    Colon,
    String(String),
    Number(i64),
    Bool(bool),
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
                b'"' => Ok(JsonTok::String(read_token(&self.parser))),
                b'0'..=b'9' | b'-' => Ok(JsonTok::Number(read_num(&self.parser))),
                thing => try_read_bool(&self.parser)
                    .ok_or(JsonParseError::UnexpectedToken((thing as char).to_string())),
            });
        }
    }
}

fn try_read_bool(parser: &Parser) -> Option<JsonTok> {
    let off = parser.offset();
    if b"true" == parser.subbytes(off - 1, off + 3) {
        parser.consume(3);
        Some(JsonTok::Bool(true))
    } else if b"false" == parser.subbytes(off - 1, off + 4) {
        parser.consume(4);
        Some(JsonTok::Bool(false))
    } else {
        None
    }
}

fn read_num(parser: &Parser) -> i64 {
    let start = parser.offset() - 1;
    parser.consume_while(|c| c.is_ascii_digit());

    parser.substr_to_offset(start).parse().unwrap()
}

/* TODO: This is failable */
fn read_token(parser: &Parser) -> String {
    let start = parser.offset() - 1;
    let mut hit_escape = false;

    while let Some(c) = parser.take() {
        match c {
            b'\\' if !hit_escape => {
                hit_escape = true;
            }
            b'"' if !hit_escape => break,
            _ => {
                hit_escape = false;
            }
        }
    }
    parser.substr(start + 1, parser.offset() - 1).to_owned()
}
