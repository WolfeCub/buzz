use buzz_types::HttpMethod;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated, token::Comma, DeriveInput, Ident};

mod route_parser;
mod routes;
mod utils;

use routes::*;
use utils::*;

#[proc_macro]
pub fn routes(input: TokenStream) -> TokenStream {
    let identifiers = parse_macro_input!(input with Punctuated::<Ident, Comma>::parse_terminated);

    let quotes = identifiers.into_iter().map(|ident| {
        let wrapper_name = make_wrapper_name(&ident);
        let metadata_name = make_metedata_name(&ident);

        quote! {
            (#wrapper_name, #metadata_name)
        }
    });

    TokenStream::from(quote! {
        vec![#(#quotes),*]
    })
}

macro_rules! generate_wrapper_macro {
    ($name:ident, $enum_method:tt) => {
        #[proc_macro_attribute]
        pub fn $name(attr: TokenStream, item: TokenStream) -> TokenStream {
            create_wrapper(HttpMethod::$enum_method, attr, item)
        }
    };
}

generate_wrapper_macro!(get, Get);
generate_wrapper_macro!(put, Put);
generate_wrapper_macro!(post, Post);
generate_wrapper_macro!(delete, Delete);
generate_wrapper_macro!(patch, Patch);
generate_wrapper_macro!(options, Options);

#[proc_macro_derive(Deserialize)]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let thing = match input.data {
        syn::Data::Struct(s) => quote! {
            struct Hello;
        },
        syn::Data::Enum(e) => quote! {
            struct Hello;
        },
        syn::Data::Union(_) => return compile_error("Cannot derive for unions"),
    };

    TokenStream::from(thing)
}
