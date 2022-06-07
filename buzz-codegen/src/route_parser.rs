use buzz_types::errors::RouteParseError;
use buzz_types::Parser;
use quote::{quote, ToTokens}; 

pub fn parse_route(attribute_path: String) -> Result<Vec<SegmentType>, RouteParseError> {
    let parser = Parser::new(attribute_path.as_bytes());

    /* TODO: Maybe this doesn't actually make sense but for now it exists */
    if Some(b'/') != parser.peek() {
        return Err(RouteParseError::MissingLeadingSlash);
    }

    /* TODO: Maybe counting the '/'s and allocating the right amount is faster? */
    let mut list = Vec::new();

    while parser.remaining() > 0 {
        parser.consume(1);
        let start = parser.offset();
        parser.consume_while(|c| c != b'/');

        let cand = parser.subbytes_to_offset(start);

        if cand[0] == b'{' && cand[cand.len() - 1] == b'}' {
            let var_name = parser.substr(start + 1, parser.offset() - 1);
            list.push(SegmentType::Variable(var_name.to_owned()))
        } else {
            let var_name = parser.substr_to_offset(start);
            list.push(SegmentType::Const(var_name.to_owned()))
        }
    }

    Ok(list)
}

#[derive(Debug)]
pub enum SegmentType {
    Const(String),
    Variable(String),
    SegNone,
}

impl ToTokens for SegmentType {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        match self {
            SegmentType::Const(seg) => tokens.extend(quote! {
                ::buzz::types::SegmentType::Const(#seg)
            }),
            SegmentType::Variable(name) => tokens.extend(quote! {
                ::buzz::types::SegmentType::Variable(#name)
            }),
            SegmentType::SegNone => tokens.extend(quote! {
                ::buzz::types::SegmentType::SegNone
            }),
        }
    }
}
