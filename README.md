# Buzz

A rust web framework that avoids dependancies wherever possible.

```rust
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

#[get("/query")] // will match ?arg=something
fn query(arg: Option<String>) -> impl Respond {
    arg
}

#[get("/context")]
fn context(context: BuzzContext) -> impl Respond {
    context.headers.get("Some-Header").map(String::from)
}

fn main() {
    Buzz::new("127.0.0.1:8080")
        .route(route!(foo))
        .route(route!(bar))
        .route(route!(it))
        .route(route!(empty))
        .route(route!(other))
        .route(route!(params))
        .route(route!(query))
        .route(route!(context))
        .run_server();
}

```
