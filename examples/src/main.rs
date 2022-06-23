use buzz::{prelude::*, json::Json};

mod other;

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

#[get("/query")]
fn query(test: Option<String>) -> impl Respond {
    test
}

#[get("/inject")]
fn inject(thing: Inject<i32>) -> impl Respond {
    thing.get().to_string()
}

#[derive(Deserialize)]
struct Thing {
    foo: String,
    bar: i64,
}

#[post("/json", body = "request_body")]
fn json(request_body: Json<Thing>) -> impl Respond {
    request_body.foo.clone()
}

fn main() {
    Buzz::new("127.0.0.1:8080")
        .routes(routes!(
            foo,
            blah,
            fooone,
            footwo,
            foothree,
            bar,
            it,
            empty,
            other,
            params,
            paramsthree,
            query,
            inject,
            json
        ))
        .router(other::router)
        .register::<i32>(42)
        .run_server();
}
