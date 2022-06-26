use once_cell::sync::OnceCell;

use buzz::prelude::*;
use buzz::{json::Json, prelude};

#[get("/simple-str")]
fn simple_returns_str() -> impl Respond {
    "simple"
}

#[get("/simple-string")]
fn simple_returns_string() -> impl Respond {
    format!("simple")
}

#[get("/simple-option-some")]
fn simple_returns_option_some() -> impl Respond {
    Some("simple")
}

#[get("/simple-option-none")]
fn simple_returns_option_none() -> impl Respond {
    Option::<()>::None
}

#[get("/simple-result-ok")]
fn simple_returns_result_ok() -> impl Respond {
    Result::<&str, &dyn std::error::Error>::Ok("simple")
}

#[get("/simple-result-err")]
fn simple_returns_result_err() -> impl Respond {
    Result::<(), _>::Err(std::fmt::Error::default())
}

#[get("/param/{name}")]
fn param_end(name: String) -> impl Respond {
    format!("end|{name}")
}

#[get("/param/{name}/end")]
fn param_middle(name: String) -> impl Respond {
    format!("middle|{name}")
}

#[get("/{name}/param")]
fn param_beginning(name: String) -> impl Respond {
    format!("beginning|{name}")
}

#[get("/query-single")]
fn query_single(name: Option<String>) -> impl Respond {
    name.map(|n| format!("single|{n}"))
}

#[get("/query-many")]
fn query_many(one: Option<String>, two: Option<String>, three: Option<String>) -> impl Respond {
    Some(format!("many|{}|{}|{}", one?, two?, three?))
}

#[get("/context-header")]
fn context_header(context: BuzzContext) -> impl Respond {
    context
        .headers
        .get("Header-Name")
        .map(|h| format!("header|{h}"))
}

#[get("/combination/{route}")]
fn combination(context: BuzzContext, route: String, optional: Option<String>) -> impl Respond {
    let header = context.headers.get("Header-Name");

    Some(format!("combination|{}|{}|{}", route, optional?, header?))
}

#[get("/combination-mixed/{route_one}/{route_two}")]
fn combination_mixed(
    query_one: Option<String>,
    route_one: String,
    context: BuzzContext,
    query_two: Option<String>,
    route_two: String,
    inject_i32: Inject<i32>,
) -> impl Respond {
    let header = context.headers.get("Header-Name");

    Some(format!(
        "combination-mixed|{}|{}|{}|{}|{}|{}",
        route_one,
        route_two,
        query_one?,
        query_two?,
        header?,
        inject_i32.get()
    ))
}

#[get("/inject-i32")]
fn inject_i32(val: Inject<i32>) -> impl Respond {
    val.get().to_string()
}

/* TODO figure out how to get rid of these clones for &String types */
#[get("/inject-string")]
fn inject_string(val: Inject<String>) -> impl Respond {
    val.clone()
}

struct TestStruct {
    prop: String,
}

#[get("/inject-struct")]
fn inject_struct(val: Inject<TestStruct>) -> impl Respond {
    val.prop.clone()
}

#[get("/query-full-path")]
fn query_full_path(name: std::option::Option<String>) -> impl Respond {
    name.map(|n| format!("full-path|{n}"))
}

use std::option;
#[get("/query-partial-path")]
fn query_partial_path(name: option::Option<String>) -> impl Respond {
    name.map(|n| format!("partial-path|{n}"))
}

#[get("/mixed-paths")]
fn mixed_paths(
    ctx: prelude::BuzzContext,
    val: prelude::Inject<i32>,
    val2: buzz::prelude::Inject<i32>,
) -> impl Respond {
    let header = ctx.headers.get("Header-Name");
    Some(format!(
        "mixed-paths|{}|{}|{}",
        header?,
        val.get(),
        val2.get()
    ))
}

#[derive(Deserialize)]
struct JsonTestStruct {
    num_i64: i64,
    string: String,
    boolean: bool,
    vector: Vec<String>,
    structure: NestedJsonTestStruct,
    option_some: Option<String>,
    option_none: Option<String>,
    option_missing: Option<String>,
}

#[derive(Deserialize)]
struct NestedJsonTestStruct {
    string2: String,
}

#[post("/json-struct", body = "b")]
fn json_struct(b: Json<JsonTestStruct>) -> impl Respond {
    format!(
        "json-struct|{}|{}|{}|{}|{}|{}|{}|{}",
        b.num_i64,
        b.string,
        b.boolean,
        b.vector.join(","),
        b.structure.string2,
        b.option_some.as_ref().unwrap(),
        b.option_none.as_ref().is_none().to_string(),
        b.option_missing.as_ref().is_none().to_string(),
    )
}

pub(crate) const CONTEXT: OnceCell<Buzz> = OnceCell::new();

pub(crate) fn make_buzz() -> Buzz {
    Buzz::new("127.0.0.1:8080")
        .routes(routes!(
            simple_returns_str,
            simple_returns_string,
            simple_returns_option_some,
            simple_returns_option_none,
            simple_returns_result_ok,
            simple_returns_result_err,
            param_end,
            param_middle,
            param_beginning,
            query_single,
            query_many,
            context_header,
            combination,
            combination_mixed,
            inject_i32,
            inject_string,
            inject_struct,
            query_full_path,
            query_partial_path,
            mixed_paths,
            json_struct,
        ))
        .register(42i32)
        .register("fourty two".to_owned())
        .register(TestStruct {
            prop: "fourty two".to_owned(),
        })
}
