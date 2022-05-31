use buzz::prelude::*;
use buzz_macro::*;

#[get("/foo")]
fn foo() -> impl Respond {
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
    Result::<(), _>::Err(std::fmt::Error::default());
}

fn main() {
    Buzz::new("127.0.0.1:8080")
        .route(route!(foo))
        .route(route!(bar))
        .route(route!(it))
        .route(route!(empty))
        .route(route!(other))
        .run_server();
}
