use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{punctuated::Punctuated, Ident, PathSegment};

pub(crate) fn compile_error(message: &str) -> TokenStream {
    TokenStream::from(quote!(compile_error!(#message)))
}

pub(crate) fn make_wrapper_name(name: &Ident) -> Ident {
    format_ident!("buzz_wrapper_{}", name)
}

pub(crate) fn make_metedata_name(name: &Ident) -> Ident {
    format_ident!("buzz_metadata_{}", name)
}

pub(crate) fn match_path<T>(
    valid_paths: &Vec<Vec<&str>>,
    matching: &Punctuated<PathSegment, T>,
) -> bool {
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
