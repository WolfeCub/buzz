use std::collections::HashMap;

use async_trait::async_trait;

use crate::{
    BuzzContext, HttpResponse, dependancy_injection::DependancyInjection, errors::BuzzError,
};

/// A wrapper around a user defined route handler.
///
/// When a user defines a function and decorates it with a macro like [`get`](buzz_codegen::get) a
/// wrapper function is generated that will call the users function with the correct arguments. That
/// wrapper function will have the type signature of [`Handler`]. It takes a [`Vec`] of route params
/// a [`HashMap`] of query params, an optional [`&str`] containing the body, a
/// [`BuzzContext`](crate::BuzzContext) which contains any other metadata the caller might need and a
/// [`DependancyInjection`](crate::dev::DependancyInjection) which can be used to fetch registered types.
#[async_trait]
pub trait Handler {
    async fn handle<'req, 'buzz>(
        &self,
        vars: Vec<&'req str>,
        query_params: HashMap<&'req str, &'req str>,
        request_body: Option<&'req str>,
        context: BuzzContext<'buzz>,
        di: &'buzz DependancyInjection,
    ) -> Result<HttpResponse, BuzzError>;

    fn clone_handler(&self) -> Box<dyn Handler>;
}

impl Clone for Box<dyn Handler> {
    fn clone(&self) -> Self {
        self.clone_handler()
    }
}
