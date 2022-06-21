use super::*;
use proptest::prelude::*;

fn json() -> impl Strategy<Value = Json> {
    let leaf = prop_oneof![
        any::<i64>().prop_map(Json::Number),
        any::<bool>().prop_map(Json::Bool),
        "[^\\\\\"]*".prop_map(Json::String),
    ];

    leaf.prop_recursive(3, 5, 3, |inner| {
        prop_oneof![
            prop::collection::vec(json(), 0..3).prop_map(Json::Array),
            prop::collection::vec(("[^\\\\\"]*", json()), 0..3).prop_map(Json::Object),
        ]
    }).boxed()
}

proptest! {
    #[test]
    fn it_parses_random_json(input in json()) {
        let input_str = &input.to_string();
        let result = Json::parse(input_str);

        assert!(result.is_ok(), "Error: {:#?}, Input str: {}", result, input_str);
        assert_eq!(result.unwrap(), input, "Input str: {}", input_str);
    }
}

#[test]
fn it_parses_escaped_json() {
    let result = Json::parse("\"\\\"\"");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Json::String("\\\"".to_owned()));
}
