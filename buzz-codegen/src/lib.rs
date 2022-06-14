use buzz_types::HttpMethod;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use route_parser::parse_route;
use syn::{
    parse_macro_input, punctuated::Punctuated, token::Comma, AngleBracketedGenericArguments,
    AttributeArgs, Ident, ItemFn, NestedMeta, PathArguments, PathSegment,
};

mod route_parser;

/* TODO: Type match and true to auto ".into()" */
fn create_wrapper(method: HttpMethod, path: &NestedMeta, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let fn_args_result = input
        .sig
        .inputs
        .iter()
        .map(|arg| {
            if let syn::FnArg::Typed(pat_type) = arg {
                if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                    if let syn::Type::Path(type_path) = &*pat_type.ty {
                        Ok((&pat_ident.ident, &type_path.path.segments))
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

    let mut route_index = 0usize;

    let option_paths = vec![vec!["std", "option", "Option"]];
    let context_paths = vec![
        vec!["buzz", "types", "BuzzContext"],
        vec!["buzz", "prelude", "BuzzContext"],
    ];
    let inject_paths = vec![
        vec!["buzz", "types", "Inject"],
        vec!["buzz", "prelude", "Inject"],
    ];

    let fn_arg_tokens = fn_args_result.unwrap().into_iter().map(|(arg_name, path)| {
        if match_path(&option_paths, &path) {
            let name = arg_name.to_string();
            quote! {
                __query_params.get(#name).map(|n| String::from(*n))
            }
        } else if match_path(&context_paths, &path) {
            quote!(__context)
        } else if match_path(&inject_paths, &path) {
            let last = path.last().expect("At least one segment in path");
            if let PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) =
                &last.arguments
            {
                let ty = args
                    .first()
                    .expect("Type checker should ensure that Inject always has one argument");
                let err_message = format!("Type was not registered to on your application");
                /* TODO: This needs a better error message.
                 * Also don't crash the program just return a proper error
                 */
                quote! {
                    Inject::new(__dependancy_injection.get::<#ty>().expect(#err_message))
                }
            } else {
                panic!("Inject was called without generic arguments");
            }
        } else {
            let tmp = quote! {
                String::from(__route_params[#route_index])
            };
            route_index += 1;
            tmp
        }
    });

    let user_route = if let NestedMeta::Lit(syn::Lit::Str(lit)) = path {
        parse_route(lit.value()).expect("Invalid route")
    } else {
        return compile_error("Argument must be a string literal");
    };

    let enum_name = format_ident!("{}", format!("{:#?}", method));
    let name = &input.sig.ident;
    let wrapper_name = make_wrapper_name(name);
    let metadata_name = make_metedata_name(name);

    let expanded = quote! {
        #input

        fn #wrapper_name(
            __route_params: Vec<&str>,
            __query_params: ::std::collections::HashMap<&str, &str>,
            __context: ::buzz::types::BuzzContext,
            __dependancy_injection: &::buzz::types::dev::DependancyInjection,
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

fn compile_error(message: &str) -> TokenStream {
    TokenStream::from(quote!(compile_error!(#message)))
}

fn make_wrapper_name(name: &Ident) -> Ident {
    format_ident!("buzz_wrapper_{}", name)
}

fn make_metedata_name(name: &Ident) -> Ident {
    format_ident!("buzz_metadata_{}", name)
}

fn match_path<T>(valid_paths: &Vec<Vec<&str>>, matching: &Punctuated<PathSegment, T>) -> bool {
    fn helper<T>(actual: &Vec<&str>, matching: &Punctuated<PathSegment, T>) -> bool {
        let mut i = 0;
        for seg in matching {
            loop {
                if i >= actual.len() {
                    return false;
                }
                if actual[i] == seg.ident.to_string() {
                    break;
                }
                i += 1;
            }
        }

        true
    }

    valid_paths.iter().any(|path| helper(path, matching))
}
