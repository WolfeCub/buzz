use crate::{HttpRequest, HttpResponse};

/// A function that can modify a request before it's handled by routing
///
/// Middleware will run in the order they are registered before the routing algorithm is applied.
/// [`Result`](std::result::Result) is returned to allow a middleware to short-circuit any further
/// processing. An `OK` variant indicates that you wish to continue processing (maybe you modified
/// the request in some way and are read for routing/handling to apply). Alternatively an `Err`
/// varient indicates that you want to return immediately. No further middleware will be processed
/// nor will any routing or handling occur.
pub type Middleware = fn(HttpRequest) -> Result<HttpRequest, HttpResponse>;
