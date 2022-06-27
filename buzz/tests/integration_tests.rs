use std::collections::HashMap;

use buzz_types::{HttpMethod, HttpRequest, HttpStatusCode};

use proptest::prelude::*;

mod mock_buzz;
use mock_buzz::*;

mod test_utils;
use test_utils::*;

#[test]
fn trybuild() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/trybuild/*.rs");
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
    let response = request!(Get, "/query-single?foo=blah&bar=some&hello=goodbye");

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
            "Header-Name": value
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
            "Header-Name": header
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
            "Header-Name": header
        );

        assert!(response.body.is_some());
        assert_eq!(response.status_code, HttpStatusCode::Ok);
        assert_eq!(response.body.unwrap(), format!(
            "combination-mixed|{route_one}|{route_two}|{query_one}|{query_two}|{header}|42"
        ));
    }
}

#[test]
fn it_responds_to_inject_i32() {
    let response = request!(Get, "/inject-i32");

    assert!(response.body.is_some());
    assert_eq!(response.status_code, HttpStatusCode::Ok);
    assert_eq!(response.body.unwrap(), "42");
}

#[test]
fn it_changes_inject_mut_i32() {
    let buzz = make_buzz();

    let check_inject_value = |val: &str| {
        let response = request!(buzz, Get, "/inject-i32");
        assert!(response.body.is_some());
        assert_eq!(response.status_code, HttpStatusCode::Ok);
        assert_eq!(response.body.unwrap(), val);
    };

    check_inject_value("42");

    let response = request!(buzz, Get, "/inject-mut-i32-change");
    assert_eq!(response.status_code, HttpStatusCode::NoContent);

    check_inject_value("77");
}

#[test]
fn it_responds_to_inject_string() {
    let response = request!(Get, "/inject-string");

    assert!(response.body.is_some());
    assert_eq!(response.status_code, HttpStatusCode::Ok);
    assert_eq!(response.body.unwrap(), "fourty two");
}

#[test]
fn it_responds_to_inject_struct() {
    let response = request!(Get, "/inject-struct");

    assert!(response.body.is_some());
    assert_eq!(response.status_code, HttpStatusCode::Ok);
    assert_eq!(response.body.unwrap(), "fourty two");
}

proptest! {
    #[test]
    fn it_responds_to_query_full_path(value in "[A-Za-z0-9-._~:#\\[\\]@!$'()*+,;=]") {
        let response = request!(Get, format!("/query-full-path?name={value}"));

        assert!(response.body.is_some());
        assert_eq!(response.status_code, HttpStatusCode::Ok);
        assert_eq!(response.body.unwrap(), format!("full-path|{value}"));
    }

    #[test]
    fn it_responds_to_query_partial_path(value in "[A-Za-z0-9-._~:#\\[\\]@!$'()*+,;=]") {
        let response = request!(Get, format!("/query-partial-path?name={value}"));

        assert!(response.body.is_some());
        assert_eq!(response.status_code, HttpStatusCode::Ok);
        assert_eq!(response.body.unwrap(), format!("partial-path|{value}"));
    }

    #[test]
    fn it_responds_to_mixed_paths(
        header in "[A-Za-z0-9-._~:#\\[\\]@!$'()*+,;=]",
    ) {

        let response = request!(
            Get, "/mixed-paths",
            "Header-Name": header.clone()
        );

        assert!(response.body.is_some());
        assert_eq!(response.status_code, HttpStatusCode::Ok);
        assert_eq!(response.body.unwrap(), format!("mixed-paths|{header}|42|42"));
    }

    #[test]
    fn it_responds_to_json_struct(
        num_i64: i64,
        string in valid_str(),
        boolean: bool,
        vector in valid_str_vec(),
        string2 in valid_str(),
        option_some in valid_str(),
    ) {
        let v = vector
            .iter()
            .map(|s| format!("\"{}\"", s))
            .collect::<Vec<_>>()
            .join(",");

        let thing = format!(r#"{{
            "num_i64": {},
            "string": "{}",
            "boolean": {},
            "vector": [{}],
            "structure": {{
                "string2": "{}"
            }},
            "option_some": "{}",
            "option_none": null
        }}"#, num_i64, string, boolean, v, string2, option_some);


        let response = request!(
            Post, "/json-struct",
            body: &thing
        );

        assert!(response.body.is_some());
        assert_eq!(response.status_code, HttpStatusCode::Ok);
        assert_eq!(
            response.body.unwrap(),
            format!("json-struct|{}|{}|{}|{}|{}|{}|true|true",
                    num_i64,
                    string,
                    boolean,
                    vector.join(","),
                    string2,
                    option_some)
        );

    }

}
