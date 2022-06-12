use proc_macro::TokenStream;
use quote::{format_ident, quote};
use route_parser::parse_route;
use syn::{parse_macro_input, AttributeArgs, Ident, ItemFn, NestedMeta};
use buzz_types::HttpMethod;

mod route_parser;

macro_rules! generate_wrapper_macro {
    ($name:ident, $enum_method:tt) => {
        #[proc_macro_attribute]
        pub fn $name(attr: TokenStream, item: TokenStream) -> TokenStream {
            let args = parse_macro_input!(attr as AttributeArgs);
            let path = &args[0];

            create_wrapper(HttpMethod::$enum_method, path, item)
        }
    };
}

generate_wrapper_macro!(get, Get);
generate_wrapper_macro!(put, Put);
generate_wrapper_macro!(post, Post);
generate_wrapper_macro!(delete, Delete);
generate_wrapper_macro!(patch, Patch);
generate_wrapper_macro!(options, Options);

fn make_wrapper_name(name: &Ident) -> Ident {
    format_ident!("buzz_wrapper_{}", name)
}

fn make_metedata_name(name: &Ident) -> Ident {
    format_ident!("buzz_metadata_{}", name)
}

/* TODO: Type match and true to auto ".into()" */
fn create_wrapper(method: HttpMethod, path: &NestedMeta, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let name = &input.sig.ident;
    let wrapper_name = make_wrapper_name(name);
    let metadata_name = make_metedata_name(name);

    let user_route = if let NestedMeta::Lit(syn::Lit::Str(lit)) = path {
        parse_route(lit.value()).expect("Invalid route")
    } else {
        return compile_error("Argument must be a string literal");
    };

    let fn_args_result = input
        .sig
        .inputs
        .iter()
        .map(|arg| {
            if let syn::FnArg::Typed(pat_type) = arg {
                if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                    if let syn::Type::Path(type_path) = &*pat_type.ty {
                        Ok((
                            &pat_ident.ident,
                            &type_path
                                .path
                                .segments
                                .last()
                                .expect("Every type has at least one segment")
                                .ident,
                        ))
                    } else {
                        Err(compile_error("Type is not a path"))
                    }
                } else {
                    Err(compile_error("Found untyped non-identifier arg"))
                }
            } else {
                Err(compile_error("Found self in args which is not allowed"))
            }
        })
        .collect::<Result<Vec<_>, _>>();

    if let Err(e) = fn_args_result {
        return TokenStream::from(e);
    }

    let fn_args = fn_args_result.unwrap();

    let mut fn_arg_tokens = vec![];
    let mut route_index = 0usize;

    for (arg_name, arg_type) in fn_args.iter().copied() {
        fn_arg_tokens.push(match arg_type.to_string().as_str() {
            "Option" => {
                let name = arg_name.to_string();
                quote! {
                    __query_params.get(#name).map(|n| String::from(*n))
                }
            }
            "BuzzContext" => {
                quote!(__context)
            }
            _ => {
                let tmp = quote! {
                    String::from(__route_params[#route_index])
                };
                route_index += 1;
                tmp
            }
        });
    }

    let enum_name = format_ident!("{}", format!("{:#?}", method));

    let expanded = quote! {
        #input

        fn #wrapper_name(
            __route_params: Vec<&str>,
            __query_params: ::std::collections::HashMap<&str, &str>,
            __context: ::buzz::types::BuzzContext,
        ) -> ::buzz::types::HttpResponse {
            #name(
                #(#fn_arg_tokens,)*
            ).respond()
        }

        #[allow(non_upper_case_globals)]
        const #metadata_name: ::buzz::types::RouteMetadata = ::buzz::types::RouteMetadata {
            route: &[#(#user_route,)*],
            method: ::buzz::types::HttpMethod::#enum_name,
        };
    };

    TokenStream::from(expanded)
}

#[proc_macro]
pub fn route(input: TokenStream) -> TokenStream {
    let id = parse_macro_input!(input as Ident);

    let wrapper_name = make_wrapper_name(&id);
    let metadata_name = make_metedata_name(&id);

    let expanded = quote! {
        (#wrapper_name, #metadata_name)
    };

    TokenStream::from(expanded)
}

fn compile_error(message: &str) -> TokenStream {
    TokenStream::from(quote!(compile_error!(#message)))
}
