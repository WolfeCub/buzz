use buzz::prelude::*;
use buzz_macro::*;

mod other;

#[get("/foo")]
fn foo() -> String {
    return "foo".to_owned();
}

#[post("/bar")]
fn bar() -> String {
    return "bar".to_owned();
}

#[delete("/it")]
fn it() -> String{
    return "it".to_owned();
}

fn main() {
    Buzz::new("127.0.0.1:8080")
        .with_attributes()
        .run_server();
}
