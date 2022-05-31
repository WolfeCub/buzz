use proc_macro::TokenStream;
use quote::{quote, quote_spanned, format_ident};
use syn::{parse_macro_input, spanned::Spanned, AttributeArgs, ItemFn, NestedMeta};

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
create_registry_macro!(post, "POST");
create_registry_macro!(put, "PUT");
create_registry_macro!(delete, "DELETE");

fn save_to_registry(method: &'static str, path: &NestedMeta, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let fn_item = &input.sig.ident;
    let const_name = format_ident!("BUZZ_{}", &input.sig.ident);

    let expanded = quote! {
        #[allow(non_upper_case_globals)]
        #[buzz::linkme::distributed_slice(BUZZ_REGISTRY)]
        static #const_name: buzz::types::HttpService = buzz::types::HttpService {
            path: #path,
            handler: #fn_item,
            method: #method,
        };
        #input
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn buzz_main(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let input = parse_macro_input!(item as ItemFn);

    let body = &input.block;

    /* TODO: Maybe don't complain about no main if this errors */
    let expanded = if args.len() != 0 {
        quote_spanned! {
            args[0].span() => compile_error!("No args expected");
        }
    } else {
        quote! {
            #[buzz::linkme::distributed_slice]
            pub static BUZZ_REGISTRY: [buzz::types::HttpService] = [..];
            fn main() {
                #body
            }
        }
    };

    TokenStream::from(expanded)
}
