use buzz::prelude::*;

#[get("/other-file")]
fn other_file() -> impl Respond {
    "other file"
}

pub fn router(builder: Buzz) -> Buzz {
    builder.routes(routes!(other_file))
}
