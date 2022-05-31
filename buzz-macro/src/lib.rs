use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, AttributeArgs, ItemFn, NestedMeta, Ident};

macro_rules! create_registry_macro {
    ($name:ident, $enum_method:literal) => {
        #[proc_macro_attribute]
        pub fn $name(attr: TokenStream, item: TokenStream) -> TokenStream {
            let args = parse_macro_input!(attr as AttributeArgs);
            let path = &args[0];

            create_wrapper($enum_method, path, item)
        }
    };
}

create_registry_macro!(get, "GET");
create_registry_macro!(put, "PUT");
create_registry_macro!(post, "POST");
create_registry_macro!(delete, "DELETE");
create_registry_macro!(patch, "PATCH");
create_registry_macro!(options, "OPTIONS");

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

    let expanded = quote! {
        #input

        fn #wrapper_name() -> buzz::types::HttpResponse {
            #name().respond()
        }

        #[allow(non_upper_case_globals)]
        static #metadata_name: buzz::dev::RouteMetadata = buzz::dev::RouteMetadata {
            method: #method,
            path: #path,
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
        (#wrapper_name, #metadata_name)
    };

    TokenStream::from(expanded)
}
