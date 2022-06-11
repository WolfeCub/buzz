#![feature(let_chains)]
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use route_parser::{parse_route, SegmentType};
use syn::{parse_macro_input, AttributeArgs, FnArg, Ident, ItemFn, NestedMeta, Pat};

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

/* TODO: Maybe not so wishy washy. Enforce that these actually compile to an enum */
generate_wrapper_macro!(get, "Get");
generate_wrapper_macro!(put, "Put");
generate_wrapper_macro!(post, "Post");
generate_wrapper_macro!(delete, "Delete");
generate_wrapper_macro!(patch, "Patch");
generate_wrapper_macro!(options, "Options");

fn make_wrapper_name(name: &Ident) -> Ident {
    format_ident!("buzz_wrapper_{}", name)
}

fn make_metedata_name(name: &Ident) -> Ident {
    format_ident!("buzz_metadata_{}", name)
}

/* TODO: Type match and true to auto ".into()" */
fn create_wrapper(method: &'static str, path: &NestedMeta, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let name = &input.sig.ident;
    let wrapper_name = make_wrapper_name(name);
    let metadata_name = make_metedata_name(name);

    let user_route = if let NestedMeta::Lit(syn::Lit::Str(lit)) = path {
        parse_route(lit.value()).expect("Invalid route")
    } else {
        return compile_error("Argument must be a string literal");
    };

    let user_route_variables: Vec<_> = user_route
        .iter()
        .filter_map(|seg| {
            if let SegmentType::Variable(name) = seg {
                Some(name)
            } else {
                None
            }
        })
        .collect();

    /* TODO: Squash this and fn_args into a single iteration */
    //for (seg_var, input_arg) in user_route_variables.iter().copied().zip(&input.sig.inputs) {
    //    if let FnArg::Typed(pat_type) = input_arg {
    //        if let Pat::Ident(pat_ident) = &*pat_type.pat {
    //            let var_name = pat_ident.ident.to_string();
    //            if *seg_var != var_name {
    //                return compile_error(&format!(
    //                    "Expected arg named `{}` but instead found: `{}`",
    //                    seg_var, var_name
    //                ));
    //            }
    //        } else {
    //            return compile_error("Found untyped non-identifier arg");
    //        }
    //    } else {
    //        return compile_error("Found self in args which is not allowed");
    //    }
    //}

    let fn_args: Vec<_> = input
        .sig
        .inputs
        .iter()
        .filter_map(|arg| {
            match arg {
                syn::FnArg::Typed(pat_type) => {
                    if let syn::Pat::Ident(pat_ident) = &*pat_type.pat && let syn::Type::Path(type_path) = &*pat_type.ty {
                        Some((&pat_ident.ident, &type_path.path.segments.last().expect("Every type has at least one segment").ident))
                    } else {
                        None
                    }
                }
                _ => None,
            }
        })
        .collect();

    let mut fn_arg_tokens = vec![];
    let mut route_index = 0usize;

    for (arg_name, arg_type) in fn_args.iter().copied() {
        fn_arg_tokens.push(if arg_type.to_string() != "Option" {
            let tmp = quote! {
                String::from(route_params[#route_index])
            };
            route_index += 1;
            tmp
        } else {
            let name = arg_name.to_string();
            quote! {
                query_params.get(#name).map(|n| String::from(*n))
            }
        });
    }

    let enum_name = format_ident!("{}", method);

    let expanded = quote! {
        #input

        fn #wrapper_name(
            request: &::buzz::types::HttpRequest,
            route_params: Vec<&str>,
            query_params: ::std::collections::HashMap<&str, &str>
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
