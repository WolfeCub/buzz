use buzz_types::HttpMethod;
use proc_macro::TokenStream;
use quote::__private::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::token::Colon2;
use syn::{
    parse_macro_input, AngleBracketedGenericArguments, AttributeArgs, ItemFn, Lit, Meta,
    MetaNameValue, NestedMeta, PathArguments, PathSegment,
};

use crate::route_parser::parse_route;
use crate::utils::*;

pub fn create_wrapper(method: HttpMethod, attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(attr as AttributeArgs);

    let user_route = if let NestedMeta::Lit(syn::Lit::Str(lit)) = &attr_args[0] {
        parse_route(lit.value()).expect("Invalid route")
    } else {
        return compile_error("Argument must be a string literal");
    };

    let body_attr = attr_args[1..].iter().find_map(|arg| {
        if let NestedMeta::Meta(Meta::NameValue(MetaNameValue {
            path: syn::Path { segments, .. },
            lit: Lit::Str(lit_str),
            ..
        })) = arg
        {
            if "body" == segments.last()?.ident.to_string() {
                return Some(lit_str.value());
            }
        }

        None
    });

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

    let fn_arg_tokens_result = fn_args_result
        .unwrap()
        .into_iter()
        .map(|(arg_name, path)| {
            let type_name = quote!(#path).to_string().replace(" ", "");
            if body_attr.is_some() && *body_attr.as_ref().unwrap() == arg_name.to_string() {
                Ok(quote! {
                    <#path as buzz::types::traits::FromBody>::from_body(
                        &__body.ok_or(::buzz::types::errors::BuzzError::BadRequest(
                            "Body was empty".to_owned()
                        ))?
                    )?
                })
            } else if match_path(&OPTION_PATHS, &path) {
                let name = arg_name.to_string();
                Ok(quote! {
                    __query_params.get(#name).map(|param| {
                        param.parse().map_err(|e| ::buzz::types::errors::BuzzError::BadRequest(
                                format!("Expected type `{}` but got '{}'", #type_name, param)
                                ))
                    }).transpose()?
                })
            } else if match_path(&CONTEXT_PATHS, &path) {
                Ok(quote!(__context))
            } else if match_path(&INJECT_PATHS, &path) || match_path(&INJECTMUT_PATHS, &path) {
                inject_handler(path)
            } else {
                let tmp = quote! {
                    {
                        let param = __route_params[#route_index];
                        param.parse().map_err(|e| ::buzz::types::errors::BuzzError::BadRequest(
                            format!("Expected type `{}` but got '{}'", #type_name, param)
                        ))?
                    }
                };
                route_index += 1;
                Ok(tmp)
            }
        })
        .collect::<Result<Vec<_>, _>>();

    if let Err(e) = fn_arg_tokens_result {
        return TokenStream::from(e);
    }

    let fn_arg_tokens = fn_arg_tokens_result.unwrap();

    let enum_name = format_ident!("{}", format!("{:#?}", method));
    let name = &input.sig.ident;
    let wrapper_name = make_wrapper_name(name);
    let metadata_name = make_metedata_name(name);

    let expanded = quote! {
        #input

        fn #wrapper_name(
            __route_params: Vec<&str>,
            __query_params: ::std::collections::HashMap<&str, &str>,
            __body: Option<&str>,
            __context: ::buzz::types::BuzzContext,
            __dependancy_injection: &::buzz::types::dev::DependancyInjection,
        ) -> Result<::buzz::types::HttpResponse, ::buzz::types::errors::BuzzError> {
            std::panic::catch_unwind(|| {
                Ok(#name(
                    #(#fn_arg_tokens,)*
                ).respond())
            }).unwrap_or_else(|_| {
                Ok(::buzz::types::HttpResponse::new(::buzz::types::HttpStatusCode::InternalServerError))
            })
        }

        #[allow(non_upper_case_globals)]
        const #metadata_name: ::buzz::types::RouteMetadata = ::buzz::types::RouteMetadata {
            route: &[#(#user_route,)*],
            method: ::buzz::types::HttpMethod::#enum_name,
        };
    };

    TokenStream::from(expanded)
}

fn inject_handler(path: &Punctuated<PathSegment, Colon2>) -> Result<TokenStream2, TokenStream> {
    let last = path.last().expect("At least one segment in path");
    if let PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) =
        &last.arguments
    {
        let ty = args
            .first()
            .expect("Type checker should ensure that Inject always has one argument");

        let ty_string = ty.to_token_stream().to_string();

        let acquire_lock = if last.ident.to_string() == "InjectMut" {
            quote! {
                .write()
                    .map_err(|_e| ::buzz::types::errors::BuzzError::LockAcquirePoisoned(
                            #ty_string.to_owned(),
                            "write".to_owned()))?
                    .downcast_mut()
            }
        } else {
            quote! {
                .read()
                    .map_err(|_e| ::buzz::types::errors::BuzzError::LockAcquirePoisoned(
                            #ty_string.to_owned(),
                            "read".to_owned()))?
                    .downcast_ref()
            }
        };

        Ok(quote! {
            <#path>::new(
                __dependancy_injection.get::<#ty>()
                .ok_or(
                    ::buzz::types::errors::BuzzError::UseOfUnregesteredInject(#ty_string.to_owned())
                    )?
                #acquire_lock
                .unwrap()
                )
        })
    } else {
        Err(compile_error("Inject was called without generic arguments"))
    }
}
