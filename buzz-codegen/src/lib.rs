use proc_macro::TokenStream;
use quote::{format_ident, quote};
use route_parser::{parse_route, SegmentType};
use syn::{parse_macro_input, AttributeArgs, Ident, ItemFn, NestedMeta, FnArg, Pat};

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

fn create_wrapper(method: &'static str, path: &NestedMeta, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let name = &input.sig.ident;
    let wrapper_name = make_wrapper_name(name);
    let metadata_name = make_metedata_name(name);

    let flat = match path {
        NestedMeta::Lit(syn::Lit::Str(lit)) => parse_route(lit.value()).expect("Invalid route"),
        _ => panic!("Argument must be a string literal"),
    };

    /* TODO: Type match and true to auto ".into()" */
    //let fargs: Vec<(&syn::Pat, &syn::Type)> = input
    //    .sig
    //    .inputs
    //    .iter()
    //    .filter_map(|arg| match arg {
    //        syn::FnArg::Typed(pat_type) => {
    //            if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
    //                Some((&*pat_type.pat, &*pat_type.ty))
    //            } else {
    //                None
    //            }
    //        }
    //        _ => None,
    //    })
    //    .collect();

    let segment_vars: Vec<_> = flat
        .iter()
        .filter_map(|seg| {
            if let SegmentType::Variable(name) = seg {
                Some(name)
            } else {
                None
            }
        })
        .collect();

    if input.sig.inputs.len() != segment_vars.len() {
        panic!("Route params and arguments must be the same length");
    }

    for (seg_var, input_arg) in segment_vars.iter().copied().zip(&input.sig.inputs) {
        if let FnArg::Typed(pat_type) = input_arg {
            if let Pat::Ident(pat_ident) = &*pat_type.pat {
                let var_name = pat_ident.ident.to_string();
                if *seg_var != var_name {
                    panic!("Expected arg named `{}` but instead found: `{}`", seg_var, var_name);
                }
            } else {
                panic!("Found untyped non-identifier arg");
            }
        } else {
            panic!("Found self in args which is not allowed");
        }
    }

    let enum_name = format_ident!("{}", method);
    let nums = 0..input.sig.inputs.len();

    let expanded = quote! {
        #input

        fn #wrapper_name(request: &::buzz::types::HttpRequest, bindings: Vec<&str>) -> ::buzz::types::HttpResponse {
            #name(
                #(String::from(bindings[#nums]),)*
            ).respond()
        }

        #[allow(non_upper_case_globals)]
        const #metadata_name: ::buzz::types::RouteMetadata = ::buzz::types::RouteMetadata {
            route: &[#(#flat,)*],
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
