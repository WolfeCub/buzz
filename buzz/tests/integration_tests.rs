use once_cell::sync::OnceCell;
use std::collections::HashMap;

use buzz::prelude::*;
use buzz_types::{HttpMethod, HttpRequest, HttpStatusCode};

use proptest::prelude::*;

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
    let header = context.headers.get("Header-Name").map(|h| String::from(h));

    Some(format!("combination|{}|{}|{}", route, optional?, header?))
}

#[get("/combination-mixed/{route_one}/{route_two}")]
fn combination_mixed(
    query_one: Option<String>,
    route_one: String,
    context: BuzzContext,
    query_two: Option<String>,
    route_two: String,
) -> impl Respond {
    let header = context.headers.get("Header-Name").map(|h| String::from(h));

    Some(format!(
        "combination-mixed|{}|{}|{}|{}|{}",
        route_one, route_two, query_one?, query_two?, header?
    ))
}

static BUZZ: OnceCell<Buzz> = OnceCell::new();

fn make_buzz() -> Buzz {
    Buzz::new("127.0.0.1:8080")
        .route(route!(simple_returns_str))
        .route(route!(simple_returns_string))
        .route(route!(simple_returns_option_some))
        .route(route!(simple_returns_option_none))
        .route(route!(simple_returns_result_ok))
        .route(route!(simple_returns_result_err))
        .route(route!(param_end))
        .route(route!(param_middle))
        .route(route!(param_beginning))
        .route(route!(query_single))
        .route(route!(query_many))
        .route(route!(context_header))
        .route(route!(combination))
        .route(route!(combination_mixed))
}

macro_rules! request {
    ($method:tt, $path:literal) => {
        request!($method, $path.to_owned(),)
    };
    ($method:tt, $path:expr) => {
        request!($method, $path,)
    };
    ($method:tt, $path:literal, $($key:literal: $value:expr),*) => {
        request!($method, $path.to_owned(), $($key: $value),*)
    };
    ($method:tt, $path:expr, $($key:literal: $value:expr),*) => {
        BUZZ.get_or_init(make_buzz).dispatch(HttpRequest {
            method: HttpMethod::$method,
            path: $path,
            version: 1.1,
            headers: HashMap::from_iter([$(($key.to_owned(), $value)),*]),
        })
    };
}

#[test]
fn it_responds_to_simple_with_str_return() {
    let response = request!(Get, "/simple-str");

    assert!(response.body.is_some());
    assert_eq!(response.status_code, HttpStatusCode::Ok);
    assert_eq!(response.body.unwrap(), "simple");
}

#[test]
fn it_responds_to_simple_with_string_return() {
    let response = request!(Get, "/simple-string");

    assert!(response.body.is_some());
    assert_eq!(response.status_code, HttpStatusCode::Ok);
    assert_eq!(response.body.unwrap(), "simple");
}

#[test]
fn it_responds_to_simple_with_option_some_return() {
    let response = request!(Get, "/simple-option-some");

    assert!(response.body.is_some());
    assert_eq!(response.status_code, HttpStatusCode::Ok);
    assert_eq!(response.body.unwrap(), "simple");
}

#[test]
fn it_responds_to_simple_with_option_none_return() {
    let response = request!(Get, "/simple-option-none");

    assert!(response.body.is_none());
    assert_eq!(response.status_code, HttpStatusCode::NotFound);
}

proptest! {
    #[test]
    fn it_responds_to_param_end(route in "[A-Za-z0-9-._~:#\\[\\]@!$&'()*+,;=]") {
        let response = request!(Get, format!("/param/{route}"));

        assert!(response.body.is_some());
        assert_eq!(response.status_code, HttpStatusCode::Ok);
        assert_eq!(response.body.unwrap(), format!("end|{route}"));
    }

    #[test]
    fn it_responds_to_param_middle(route in "[A-Za-z0-9-._~:#\\[\\]@!$&'()*+,;=]") {
        let response = request!(Get, format!("/param/{route}/end"));

        assert!(response.body.is_some());
        assert_eq!(response.status_code, HttpStatusCode::Ok);
        assert_eq!(response.body.unwrap(), format!("middle|{route}"));
    }

    #[test]
    fn it_responds_to_param_beginning(route in "[A-Za-z0-9-._~:#\\[\\]@!$&'()*+,;=]") {
        let response = request!(Get, format!("/{route}/param"));

        assert!(response.body.is_some());
        assert_eq!(response.status_code, HttpStatusCode::Ok);
        assert_eq!(response.body.unwrap(), format!("beginning|{route}"));
    }

    #[test]
    fn it_responds_to_query_single_no_slash(value in "[A-Za-z0-9-._~:#\\[\\]@!$'()*+,;=]") {
        let response = request!(Get, format!("/query-single?name={value}"));

        assert!(response.body.is_some());
        assert_eq!(response.status_code, HttpStatusCode::Ok);
        assert_eq!(response.body.unwrap(), format!("single|{value}"));
    }

    #[test]
    fn it_responds_to_query_single_with_slash(value in "[A-Za-z0-9-._~:#\\[\\]@!$'()*+,;=]") {
        let response = request!(Get, format!("/query-single/?name={value}"));

        assert!(response.body.is_some());
        assert_eq!(response.status_code, HttpStatusCode::Ok);
        assert_eq!(response.body.unwrap(), format!("single|{value}"));
    }

}

#[test]
fn it_responds_to_query_single_no_params() {
    let response = request!(Get, "/query-single");

    assert!(response.body.is_none());
    assert_eq!(response.status_code, HttpStatusCode::NotFound);
}

#[test]
fn it_responds_to_query_single_wrong_params() {
    let response = request!(
        Get,
        format!("/query-single?foo=blah&bar=some&hello=goodbye")
    );

    assert!(response.body.is_none());
    assert_eq!(response.status_code, HttpStatusCode::NotFound);
}

proptest! {
    #[test]
    fn it_responds_to_many_query_in_order(
        value1 in "[A-Za-z0-9-._~:#\\[\\]@!$'()*+,;=]",
        value2 in "[A-Za-z0-9-._~:#\\[\\]@!$'()*+,;=]",
        value3 in "[A-Za-z0-9-._~:#\\[\\]@!$'()*+,;=]",
    ) {
        let response = request!(Get, format!("/query-many/?one={value1}&two={value2}&three={value3}"));

        assert!(response.body.is_some());
        assert_eq!(response.status_code, HttpStatusCode::Ok);
        assert_eq!(response.body.unwrap(), format!("many|{value1}|{value2}|{value3}"));
    }

    #[test]
    fn it_responds_to_many_query_shuffled_order(
        value1 in "[A-Za-z0-9-._~:#\\[\\]@!$'()*+,;=]",
        value2 in "[A-Za-z0-9-._~:#\\[\\]@!$'()*+,;=]",
        value3 in "[A-Za-z0-9-._~:#\\[\\]@!$'()*+,;=]",
    ) {
        let response = request!(Get, format!("/query-many/?two={value2}&three={value3}&one={value1}"));

        assert!(response.body.is_some());
        assert_eq!(response.status_code, HttpStatusCode::Ok);
        assert_eq!(response.body.unwrap(), format!("many|{value1}|{value2}|{value3}"));
    }

    #[test]
    fn it_responds_to_context_with_header(value in "[A-Za-z0-9-._~:#\\[\\]@!$'()*+,;=]") {
        let response = request!(
            Get, "/context-header",
            "Header-Name": value.clone()
        );

        assert!(response.body.is_some());
        assert_eq!(response.status_code, HttpStatusCode::Ok);
        assert_eq!(response.body.unwrap(), format!("header|{value}"));
    }

    #[test]
    fn it_responds_to_combination(
        route in "[A-Za-z0-9-._~:#\\[\\]@!$&'()*+,;=]",
        query in "[A-Za-z0-9-._~:#\\[\\]@!$'()*+,;=]",
        header in "[A-Za-z0-9-._~:#\\[\\]@!$'()*+,;=]",
    ) {
        let response = request!(
            Get, format!("/combination/{route}?optional={query}"),
            "Header-Name": header.clone()
        );

        assert!(response.body.is_some());
        assert_eq!(response.status_code, HttpStatusCode::Ok);
        assert_eq!(response.body.unwrap(), format!("combination|{route}|{query}|{header}"));
    }

    #[test]
    fn it_responds_to_combination_mixed(
        route_one in "[A-Za-z0-9-._~:#\\[\\]@!$&'()*+,;=]",
        route_two in "[A-Za-z0-9-._~:#\\[\\]@!$&'()*+,;=]",
        query_one in "[A-Za-z0-9-._~:#\\[\\]@!$'()*+,;=]",
        query_two in "[A-Za-z0-9-._~:#\\[\\]@!$'()*+,;=]",
        header in "[A-Za-z0-9-._~:#\\[\\]@!$'()*+,;=]",
    ) {

        let response = request!(
            Get, format!("/combination-mixed/{route_one}/{route_two}?query_one={query_one}&query_two={query_two}"),
            "Header-Name": header.clone()
        );

        assert!(response.body.is_some());
        assert_eq!(response.status_code, HttpStatusCode::Ok);
        assert_eq!(response.body.unwrap(), format!(
            "combination-mixed|{route_one}|{route_two}|{query_one}|{query_two}|{header}"
        ));
    }
}