/*!
A rust web framework that avoids dependancies wherever possible.

Here we return a [`&str`] and it behaves as expected. We can register a route by using
the [`routes`](crate::buzz::Buzz::routes) method and the [`routes!`](crate::codegen::routes) macro.
```no_run
# use buzz::prelude::*;
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

[`Option`] variants work as long as the inner element implements [`Respond`](crate::types::traits::Respond).
```no_run
# use buzz::prelude::*;
#[get("/it")]
fn it() -> impl Respond {
    Some("it")
}
```

Returning `None` gives you a 404.
```no_run
# use buzz::prelude::*;
#[get("/empty")]
fn empty() -> impl Respond {
    Option::<()>::None
}
```

Returning an the `Err` variant of [`Result`] gives you a 500 with the content `Err`.
```no_run
# use buzz::prelude::*;
#[get("/other")]
fn other() -> impl Respond {
    Result::<(), _>::Err(std::fmt::Error::default())
}
```

Query parameters are just regular arguments. We can specify anything that implements [`std::str::FromStr`]
```no_run
# use buzz::prelude::*;
#[get("/params/{hello}/{number}")]
fn params(hello: String, number: i32) -> impl Respond {
    format!("{hello} {number}")
}
```

Query params are also just args but are [`Option`].
In the case of `?arg=something` arg here will be `Some("something")`
and `None` if `arg` is not present in the query string.
```no_run
# use buzz::prelude::*;
#[get("/query")]
fn query(arg: Option<String>) -> impl Respond {
    arg
}
```

[`BuzzContext`](crate::types::BuzzContext) can be injected anywhere and get extra info about the request.
Here we grab an arbitrary header to make use of.
```no_run
# use buzz::prelude::*;
#[get("/context")]
fn context(context: BuzzContext) -> impl Respond {
    context.headers.get("Some-Header").map(|h| (*h).to_owned())
}
```

Registered params can be injected anywhere with the special type Inject.
```no_run
# use buzz::prelude::*;
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

Here we derive [`Deserialize`](crate::types::traits::Deserialize) for `Task` allowing us to map json or other
data types to it. We tell buzz which request param to inject the body for by specifying `body = "arg_name"`.
Buzz knows to deserialize the incoming request as JSON since we wrapped our variable in the [`Json`](crate::json::Json) type.
```no_run
# use buzz::{prelude::*, json::Json};
#[derive(Deserialize)]
struct Task {
    index: i64,
    content: String,
}

#[post("/json", body = "request_body")]
fn json(request_body: Json<Task>) -> impl Respond {
    format!("{}. {}", request_body.index, request_body.content)
}
```
*/

mod http_parse;
mod buzz;
mod routes;
pub mod json;

pub use buzz_types as types;
pub use buzz_codegen as codegen;

pub mod prelude {
    pub use super::buzz::Buzz;
    pub use super::types::traits::Respond;
    pub use super::types::{BuzzContext, Inject};
    pub use super::codegen::*;
}
