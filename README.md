# Buzz

<!-- cargo-rdme start -->

A rust web framework that avoids dependancies wherever possible.

Here we return a [`&str`] and it behaves as expected. We can register a route by using
the `routes` method and the `routes!` macro.
```rust
#[get("/foo")]
fn foo() -> impl Respond {
    "foo"
}

fn main() {
    Buzz::new("127.0.0.1:8080")
        .routes(routes!(foo))
        .run_server();
}
```

[`Option`] variants work as long as the inner element implements `Respond`.
```rust
#[get("/it")]
fn it() -> impl Respond {
    Some("it")
}
```

Returning [`None`](Option::None) gives you a 404.
```rust
#[get("/empty")]
fn empty() -> impl Respond {
    Option::<()>::None
}
```

Returning an the `Err` variant of [`Result`] gives you a 500 with the content `Err`.
```rust
#[get("/other")]
fn other() -> impl Respond {
    Result::<(), _>::Err(std::fmt::Error::default())
}
```

Query parameters are just regular arguments. We can specify anything that implements [`std::str::FromStr`]
```rust
#[get("/params/{hello}/{number}")]
fn params(hello: String, number: i32) -> impl Respond {
    format!("{hello} {number}")
}
```

Query params are also just args but are [`Option`].
In the case of `?arg=something` arg here will be `Some("something")`
and [`None`](Option::None) if `arg` is not present in the query string.
```rust
#[get("/query")]
fn query(arg: Option<String>) -> impl Respond {
    arg
}
```

`BuzzContext` can be injected anywhere and get extra info about the request.
Here we grab an arbitrary header to make use of.
```rust
#[get("/context")]
fn context(context: BuzzContext) -> impl Respond {
    context.headers.get("Some-Header").map(|h| (*h).to_owned())
}
```

Registered params can be injected anywhere with the special type Inject.
```rust
#[get("/inject")]
fn inject(val: Inject<i32>) -> impl Respond {
    val.get().to_string()
}

fn main() {
    Buzz::new("127.0.0.1:8080")
        .routes(routes!(inject))
        // Here we register a type that can be injected
        .register::<i32>(42)
        .run_server();
}
```

Here we derive `Deserialize` for `Task` allowing us to map json or other
data types to it. We tell buzz which request param to inject the body for by specifying `body = "arg_name"`.
Buzz knows to deserialize the incoming request as JSON since we wrapped our variable in the `Json` type.
[`Option`] fields can either be absent or `null`. Both will deserialize to [`None`](Option::None)
```rust
#[derive(Deserialize)]
struct Task {
    index: i64,
    content: String,
    extra: Option<String>,
}

#[post("/json", body = "request_body")]
fn json(request_body: Json<Task>) -> impl Respond {
    format!("{}. {}", request_body.index, request_body.content)
}
```

<!-- cargo-rdme end -->
