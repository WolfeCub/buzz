use buzz::prelude::*;

#[get("/foo")]
fn foo() -> impl Respond {
    "foo"
}

#[post("/foo")]
fn blah() -> impl Respond {
    "blah"
}

#[get("/foo/one")]
fn fooone() -> impl Respond {
    "foo"
}

#[get("/foo/two")]
fn footwo() -> impl Respond {
    "foo"
}

#[get("/foo/three/four")]
fn foothree() -> impl Respond {
    "foo"
}

#[get("/bar")]
fn bar() -> impl Respond {
    format!("bar")
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

#[get("/params/{goodbye}/const")]
fn paramsthree(goodbye: String) -> impl Respond {
    goodbye
}

fn main() {
    Buzz::new("127.0.0.1:8080")
        .route(route!(foo))
        .route(route!(blah))
        .route(route!(fooone))
        .route(route!(footwo))
        .route(route!(foothree))
        .route(route!(bar))
        .route(route!(it))
        .route(route!(empty))
        .route(route!(other))
        .route(route!(params))
        .route(route!(paramsthree))
        .run_server();
}
