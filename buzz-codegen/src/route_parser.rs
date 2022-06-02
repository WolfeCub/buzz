use buzz_types::Parser;
use buzz_types::errors::RouteParseError;
use quote::{quote, ToTokens};

pub fn parse_route(attribute_path: String) -> Result<Route, RouteParseError> {
    let mut parser = Parser::new(attribute_path.as_bytes());

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
            thing.push(var_name.to_owned());
        }
    }

    Ok(Route {
        path: attribute_path,
        variables: thing,
    })
}

pub struct Route {
    pub path: String,
    pub variables: Vec<String>,
}

impl ToTokens for Route {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        let path = &self.path;
        let variables = &self.variables;

        let expanded = quote! {
            ::buzz::types::Route {
                path: #path,
                variables: vec![#(#variables,)*],
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
