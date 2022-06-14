use std::collections::HashMap;

use crate::{HttpResponse, BuzzContext, dependancy_injection::DependancyInjection};

/// A wrapper around a user defined route handler.
///
/// When a user defines a function and decorates it with a macro like [`get`](buzz_codegen::get) a
/// wrapper function is generated that will call the users function with the correct arguments. That
/// wrapper function will have the type signature of [`Handler`]. It takes a [`Vec`] of route params
/// a [`HashMap`] of query params, a [`BuzzContext`](crate::BuzzContext) which contains any other
/// metadata the caller might need and a [`DependancyInjection`](crate::dev::DependancyInjection)
/// which can be used to fetch registered types.
pub type Handler = fn(Vec<&str>, HashMap<&str, &str>, BuzzContext, &DependancyInjection) -> HttpResponse;
