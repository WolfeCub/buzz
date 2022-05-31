use buzz::Buzz;
use buzz_macro::*;

#[get("/foo")]
fn foo() -> String {
    return "foo".to_owned();
}

#[get("/bar")]
fn bar() -> String {
    return "bar".to_owned();
}

#[get("/it")]
fn it() -> String{
    return "it".to_owned();
}

#[buzz_main]
fn main() {
    Buzz::new("127.0.0.1:8080")
        .with_attributes(&BUZZ_REGISTRY)
        .run_server();
}
