use buzz::prelude::*;
use buzz_macro::get;

#[get("/different")]
fn different() -> String {
    return "different".to_owned();
}
