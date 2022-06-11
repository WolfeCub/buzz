use std::collections::HashMap;

use crate::{HttpResponse, BuzzContext};

/// A wrapper around a user defined route handler.
///
/// When a user defines a function and decorates it with a macro like [`get`](buzz_codegen::get) a
/// wrapper function is generated that will call the users function with the correct arguments. That
/// wrapper function will have the type signature of [`Handler`]. It takes a [`Vec`] of route params
/// a [`HashMap`] of query params and a [`BuzzContext`](crate::BuzzContext) which contains any other
/// metadata the caller might need.
pub type Handler = fn(Vec<&str>, HashMap<&str, &str>, BuzzContext) -> HttpResponse;
