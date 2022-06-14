use buzz::prelude::*;

#[get("/non-ident-arg")]
fn non_ident_arg1("literal": &str) {
}

#[get("/non-ident-arg")]
fn non_ident_arg2((0, 1): (i32, i32)) {
}

#[get("/non-ident-arg")]
fn non_ident_arg3(_: i32) {
}

fn main() {}
