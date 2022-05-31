use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, AttributeArgs, ItemFn, NestedMeta};

macro_rules! create_registry_macro {
    ($name:ident, $enum_method:literal) => {
#[proc_macro_attribute]
pub fn $name(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let path = &args[0];

    save_to_registry($enum_method, path, item)
}

    }
}

create_registry_macro!(get, "GET");
create_registry_macro!(put, "PUT");
create_registry_macro!(post, "POST");
create_registry_macro!(delete, "DELETE");
create_registry_macro!(patch, "PATCH");
create_registry_macro!(options, "OPTIONS");

fn save_to_registry(method: &'static str, path: &NestedMeta, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let fn_item = &input.sig.ident;
    let const_name = format_ident!("BUZZ_{}", &input.sig.ident);

    let expanded = quote! {
        #[allow(non_upper_case_globals)]
        #[buzz::dev::linkme::distributed_slice(BUZZ_REGISTRY)]
        static #const_name: buzz::types::HttpService = buzz::types::HttpService {
            path: #path,
            handler: #fn_item,
            method: #method,
        };
        #input
    };

    TokenStream::from(expanded)
}
