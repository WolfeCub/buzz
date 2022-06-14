/*!
A rust web framework that avoids dependancies wherever possible.

```no_run
use buzz::prelude::*;

// &str behaves as expected
#[get("/foo")]
fn foo() -> impl Respond {
    "foo"
}

// Some varients work as long as the inside implements Respond
#[get("/it")]
fn it() -> impl Respond {
    Some("it")
}

// Returning None gives you a 404
#[get("/empty")]
fn empty() -> impl Respond {
    Option::<()>::None
}

// Returning an Err gives you a 500 with the corresponding Err content
#[get("/other")]
fn other() -> impl Respond {
    Result::<(), _>::Err(std::fmt::Error::default())
}

// Query parameters are just regular arguments
#[get("/params/{hello}")]
fn params(hello: String) -> impl Respond {
    hello
}

// Query params are also just args but are Optional
// In the case of ?arg=something arg here will be Some("something")
#[get("/query")]
fn query(arg: Option<String>) -> impl Respond {
    arg
}

// BuzzContext can be injected anywhere and get extra info about the request
#[get("/context")]
fn context(context: BuzzContext) -> impl Respond {
    context.headers.get("Some-Header").map(String::from)
}

// Registered params can be injected anywhere with the special type Inject
#[get("/inject")]
fn inject(val: Inject<i32>) -> impl Respond {
    val.get().to_string()
}

fn main() {
    Buzz::new("127.0.0.1:8080")
        // Here we register all our routes
        .routes(routes!(foo, it, empty, other, params, query, context))
        // Here we register a type that can be injected
        .register::<i32>(42)
        .run_server();
}
```
*/

mod http_parse;
mod buzz;
mod routes;

pub use buzz_types as types;

pub mod prelude {
    pub use super::buzz::Buzz;
    pub use buzz_types::traits::Respond;
    pub use buzz_types::{BuzzContext, Inject};
    pub use buzz_codegen::*;
}
