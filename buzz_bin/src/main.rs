use buzz::prelude::*;
use buzz_macro::*;

#[get("/foo")]
fn foo() -> impl Respond {
    return "foo";
}

fn main() {
    Buzz::new("127.0.0.1:8080")
        .route(route!(foo))
        .run_server();
}
