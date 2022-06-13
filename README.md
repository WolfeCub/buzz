# Buzz

<!-- cargo-rdme start -->

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

<!-- cargo-rdme end -->
