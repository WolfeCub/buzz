use buzz::{json::Json, prelude::*};

// mod other;

#[get("/foo")]
async fn foo() -> impl Respond {
    "foo"
}

#[post("/foo")]
async fn blah() -> impl Respond {
    "blah"
}

#[get("/foo/one")]
async fn fooone() -> impl Respond {
    "foo"
}

#[get("/foo/two")]
async fn footwo() -> impl Respond {
    "foo"
}

#[get("/foo/three/four")]
async fn foothree() -> impl Respond {
    "foo"
}

#[get("/bar")]
async fn bar() -> impl Respond {
    format!("bar")
}

#[get("/it")]
async fn it() -> impl Respond {
    Some("it")
}

#[get("/empty")]
async fn empty() -> impl Respond {
    Option::<()>::None
}

#[get("/other")]
async fn other() -> impl Respond {
    Result::<(), _>::Err(std::fmt::Error::default())
}

#[get("/params/{hello}")]
async fn params(hello: String) -> impl Respond {
    hello
}

#[get("/params/{goodbye}/const")]
async fn paramsthree(goodbye: String) -> impl Respond {
    goodbye
}

#[get("/query")]
async fn query(test: Option<String>) -> impl Respond {
    test
}

#[get("/inject")]
async fn inject(thing: Inject<'_, i32>) -> impl Respond {
    thing.to_string()
}

#[get("/inject-mut")]
async fn inject_mut(mut thing: InjectMut<i32>) -> impl Respond {
    *thing = 77;
}

#[derive(Deserialize)]
struct Thing {
    foo: Option<String>,
    bar: i64,
}

#[post("/json", body = "request_body")]
async fn json(request_body: Json<Thing>) -> impl Respond {
    format!(
        "{} {}",
        request_body.foo.clone().unwrap_or("None".to_owned()),
        request_body.bar
    )
}

#[get("/cast/{route}")]
async fn cast(route: i32, query: Option<i32>) -> impl Respond {
    format!("{} {}", route, query.unwrap_or(0))
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
            inject_mut,
            json,
            cast,
        ))
        // .router(other::router)
        .register::<i32>(42)
        .run_server();
}
