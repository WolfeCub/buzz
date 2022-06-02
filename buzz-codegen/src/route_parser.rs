use buzz_types::errors::RouteParseError;
use buzz_types::Parser;
use quote::{quote, ToTokens}; 

pub fn parse_route(attribute_path: String) -> Result<Route, RouteParseError> {
    let parser = Parser::new(attribute_path.as_bytes());

    /* TODO: Maybe this doesn't actually make sense but for now it exists */
    if Some(b'/') != parser.peek() {
        return Err(RouteParseError::MissingLeadingSlash);
    }

    /* TODO: Maybe counting the '/'s and allocating the right amount is faster? */
    let mut thing = Vec::new();

    while parser.remaining() > 0 {
        parser.consume(1);
        let start = parser.offset();
        parser.consume_while(|c| c != b'/');

        let cand = parser.subbytes_to_offset(start);

        if cand[0] == b'{' && cand[cand.len() - 1] == b'}' {
            let var_name = parser.substr(start + 1, parser.offset() - 1);
            thing.push(SegmentType::Variable(var_name.to_owned()));
        } else {
            let var_name = parser.substr_to_offset(start);
            thing.push(SegmentType::Const(var_name.to_owned()));
        }
    }

    Ok(Route {
        path: attribute_path,
        segments: thing,
    })
}

pub enum SegmentType {
    Const(String),
    Variable(String),
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
        }
    }
}

pub struct Route {
    pub path: String,
    pub segments: Vec<SegmentType>,
}

impl ToTokens for Route {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        let path = &self.path;
        let segments = &self.segments;

        let expanded = quote! {
            ::buzz::types::Route {
                path: #path,
                segments: &[#(#segments,)*]
            }
        };

        tokens.extend(expanded);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn thing() {
        parse_route("/blah/{thing}");
    }
}
