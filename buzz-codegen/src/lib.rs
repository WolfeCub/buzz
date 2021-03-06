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
    let name = input.ident;

    let thing = match input.data {
        syn::Data::Struct(s) => {
            let pairs = match &s.fields {
                syn::Fields::Named(fields_named) => fields_named
                    .named
                    .iter()
                    .filter_map(|field| Some((field.ident.as_ref()?, &field.ty)))
                    .collect::<Vec<_>>(),
                syn::Fields::Unnamed(_) => todo!(),
                syn::Fields::Unit => todo!(),
            };

            let mut option_idents = Vec::new();
            let mut match_cases = Vec::new();
            let mut key_count = 0usize;

            for (ident, ty) in pairs {
                let name = ident.to_string();

                let increase_count = match ty {
                    syn::Type::Path(path) if match_path(&OPTION_PATHS, &path.path.segments) => {
                        option_idents.push(ident);
                        quote!()
                    },
                    _ => {
                        key_count += 1;
                        quote!(count += 1;)
                    },
                };

                match_cases.push(quote! {
                    #name => {
                        ::std::ptr::addr_of_mut!((*ptr).#ident).write(
                            <#ty as ::buzz::types::traits::Deserialize<::buzz::json::JsonValue>>::deserialize(v)?
                        );
                        #increase_count
                    }
                });
            }

            quote! {
                impl ::buzz::types::traits::Deserialize<::buzz::json::JsonValue> for #name {
                    fn deserialize(value: ::buzz::json::JsonValue) -> Result<#name, ::buzz::types::errors::DeserializationError> {
                        let mut uninit: ::std::mem::MaybeUninit<#name> = ::std::mem::MaybeUninit::uninit();
                        let ptr = uninit.as_mut_ptr();

                        unsafe {
                            #({
                                ::std::ptr::addr_of_mut!((*ptr).#option_idents).write(None);
                            })*

                            match value {
                                ::buzz::json::JsonValue::Object(pairs) => {
                                    let mut count = 0;
                                    for (k, v) in pairs {
                                        match k.as_str() {
                                            #(#match_cases,)*
                                            _ => {},
                                        }
                                    }

                                    /* TODO: Be more specific about what keys are missing */
                                    if #key_count != count {
                                        return Err(
                                            ::buzz::types::errors::DeserializationError::MissingValues(#key_count, count)
                                        );
                                    }

                                },
                                _ => panic!("Die horribly"),
                            }

                            Ok(uninit.assume_init())
                        }
                    }
                }
            }
        }
        syn::Data::Enum(_) => quote! {
            return compile_error("Cannot derive for enums")
        },
        syn::Data::Union(_) => return compile_error("Cannot derive for unions"),
    };

    TokenStream::from(thing)
}
