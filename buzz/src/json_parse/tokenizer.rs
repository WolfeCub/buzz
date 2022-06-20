use buzz_types::dev::Parser;

#[derive(Debug)]
pub (crate) enum JsonTok {
    OpenCurly,
    CloseCurly,
    OpenSquare,
    CloseSquare,
    Quote,
    Comma,
    Colon,
    String(String),
    Number(i64),
    Bool(bool),
}

pub (crate) fn tokenize(input: &str) -> Vec<JsonTok> {
    let mut result = Vec::new();

    let parser = Parser::new(input.as_bytes());

    while let Some(c) = parser.take() {
        match c {
            b' ' => None,
            b'\t' => None,
            b'\n' => None,
            b'\r' => None,

            b'{' => Some(JsonTok::OpenCurly),
            b'}' => Some(JsonTok::CloseCurly),
            b'[' => Some(JsonTok::OpenSquare),
            b']' => Some(JsonTok::CloseSquare),
            b'"' => Some(JsonTok::Quote),
            b',' => Some(JsonTok::Comma),
            b':' => Some(JsonTok::Colon),
            b'0'..=b'9' => Some(JsonTok::Number(read_num(&parser))),
            _ => {
                let bool_val = try_read_bool(&parser);
                if bool_val.is_some() {
                    bool_val
                } else {
                    Some(JsonTok::String(read_token(&parser)))
                }
            }
        }
        .map(|tok| result.push(tok));
    }

    result
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

fn read_token(parser: &Parser) -> String {
    let start = parser.offset() - 1;
    let mut hit_escape = false;

    while let Some(c) = parser.take() {
        match c {
            b'\\' => {
                hit_escape = true;
                continue;
            }
            b'"' if hit_escape == false => break,
            _ => {
                hit_escape = false;
            }
        }
    }

    parser.substr(start, parser.offset() - 1).to_owned()
}
