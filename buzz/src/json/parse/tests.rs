use super::*;
use proptest::prelude::*;

fn json() -> impl Strategy<Value = JsonValue> {
    let leaf = prop_oneof![
        any::<i64>().prop_map(JsonValue::Number),
        any::<f64>()
            .prop_filter("Non whole numbers", |n| n - n.round() != 0.0)
            .prop_map(JsonValue::Fraction),
        any::<bool>().prop_map(JsonValue::Bool),
        "[^\\\\\"]*".prop_map(JsonValue::String),
        Just(JsonValue::Null),
    ];

    leaf.prop_recursive(3, 5, 3, |inner| {
        prop_oneof![
            prop::collection::vec(json(), 0..3).prop_map(JsonValue::Array),
            prop::collection::vec(("[^\\\\\"]*", json()), 0..3).prop_map(JsonValue::Object),
        ]
    })
    .boxed()
}

proptest! {
    #[test]
    fn it_parses_random_json(input in json()) {
        let input_str = &input.to_string();
        let result = JsonValue::parse(input_str);

        assert!(result.is_ok(), "Error: {:#?}, Input str: {}", result, input_str);
        assert_eq!(result.unwrap(), input, "Input str: {}", input_str);
    }
}

#[test]
fn it_parses_escaped_json() {
    let result = JsonValue::parse("\"\\\"\"");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), JsonValue::String("\\\"".to_owned()));
}
