use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use route_parser::{parse_route, SegmentType};
use syn::{parse_macro_input, AttributeArgs, Ident, ItemFn, NestedMeta, PatType};

mod route_metadata;
mod route_parser;

macro_rules! generate_wrapper_macro {
    ($name:ident, $enum_method:literal) => {
        #[proc_macro_attribute]
        pub fn $name(attr: TokenStream, item: TokenStream) -> TokenStream {
            let args = parse_macro_input!(attr as AttributeArgs);
            let path = &args[0];

            create_wrapper($enum_method, path, item)
        }
    };
}

generate_wrapper_macro!(get, "GET");
generate_wrapper_macro!(put, "PUT");
generate_wrapper_macro!(post, "POST");
generate_wrapper_macro!(delete, "DELETE");
generate_wrapper_macro!(patch, "PATCH");
generate_wrapper_macro!(options, "OPTIONS");

fn make_wrapper_name(name: &Ident) -> Ident {
    format_ident!("buzz_wrapper_{}", name)
}

fn make_metadata_name(name: &Ident) -> Ident {
    format_ident!("buzz_route_metadata_{}", name)
}

fn create_wrapper(method: &'static str, path: &NestedMeta, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let name = &input.sig.ident;
    let wrapper_name = make_wrapper_name(name);
    let metadata_name = make_metadata_name(name);

    let route = match path {
        NestedMeta::Lit(syn::Lit::Str(lit)) => parse_route(lit.value()).expect("Invalid route"),
        _ => panic!("Argument must be a string literal"),
    };

    let fargs: Vec<(&syn::Pat, &syn::Type)> = input
        .sig
        .inputs
        .iter()
        .filter_map(|arg| match arg {
            syn::FnArg::Typed(pat_type) => {
                if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                    Some((&*pat_type.pat, &*pat_type.ty))
                } else {
                    None
                }
            }
            _ => None,
        })
        .collect();

    let segments = &route.segments;
    let segment_vars: Vec<_> = route
        .segments
        .iter()
        .filter_map(|seg| {
            if let SegmentType::Variable(name) = seg {
                Some(name)
            } else {
                None
            }
        })
        .collect();

    if fargs.len() != segment_vars.len() {
        panic!("Route params and arguments must be the same length");
    }

    let expanded = quote! {
        #input

        fn #wrapper_name(request: ::buzz::types::HttpRequest) -> ::buzz::types::HttpResponse {
            let segments = vec![#(#segments,)*];

            #name(#(request.match_route_param(segments, #segment_vars).expect("Could not match route param"))*).respond()
        }

        #[allow(non_upper_case_globals)]
        static #metadata_name: ::buzz::types::RouteMetadata = ::buzz::types::RouteMetadata {
            method: #method,
            route: #route,
        };
    };

    TokenStream::from(expanded)
}

#[proc_macro]
pub fn route(input: TokenStream) -> TokenStream {
    let id = parse_macro_input!(input as Ident);

    let wrapper_name = make_wrapper_name(&id);
    let metadata_name = make_metadata_name(&id);

    let expanded = quote! {
        (#wrapper_name, &#metadata_name)
    };

    TokenStream::from(expanded)
}

fn var_type_token(pat: &syn::Pat, ty: &syn::Type) -> quote::__private::TokenStream {
    quote! {
        #pat: #ty
    }
}
