# Buzz

<!-- cargo-rdme start -->

A rust web framework that avoids dependancies wherever possible.

```rust
use buzz::{prelude::*, json::Json};

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

// Here we derive deserialize for `Task` allowing us to map json or other
// data types to it
#[derive(Deserialize)]
struct Task {
    index: i64,
    content: String,
}

// We tell buzz which request param to inject the body for
// It knows to deserialize the incoming request as JSON because we
// wrapped our variable in the `Json` type
#[post("/json", body = "request_body")]
fn json(request_body: Json<Task>) -> impl Respond {
    format!("{}. {}", request_body.index, request_body.content)
}

fn main() {
    Buzz::new("127.0.0.1:8080")
        // Here we register all our routes
        .routes(routes!(foo, it, empty, other, params, query, context, json))
        // Here we register a type that can be injected
        .register::<i32>(42)
        .run_server();
}
```

<!-- cargo-rdme end -->
