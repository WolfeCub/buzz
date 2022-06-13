/*!
A rust web framework that avoids dependancies wherever possible.

```no_run
use buzz::prelude::*;

#[get("/foo")]
fn foo() -> impl Respond {
    "foo"
}

#[get("/it")]
fn it() -> impl Respond {
    Some("it")
}

#[get("/empty")]
fn empty() -> impl Respond {
    Option::<()>::None
}

#[get("/other")]
fn other() -> impl Respond {
    Result::<(), _>::Err(std::fmt::Error::default())
}

#[get("/params/{hello}")]
fn params(hello: String) -> impl Respond {
    hello
}

#[get("/query")]
fn query(arg: Option<String>) -> impl Respond {
    // will capture ?arg=something
    arg
}

#[get("/context")]
fn context(context: BuzzContext) -> impl Respond {
    context.headers.get("Some-Header").map(String::from)
}

fn main() {
    Buzz::new("127.0.0.1:8080")
        .routes(routes!(foo, it, empty, other, params, query, context))
        .run_server();
}
```
*/

mod http_parse;
mod buzz;

pub use buzz_types as types;

pub mod prelude {
    pub use super::buzz::Buzz;
    pub use buzz_types::traits::Respond;
    pub use buzz_types::BuzzContext;
    pub use buzz_codegen::*;
}
