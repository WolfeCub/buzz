use super::*;

#[test]
fn it_does_thing() {
    dbg!(tokenize(r#"{
      "key": "value",
      "num": 42,
      "bool": true,
      "arr": [1, 2, 3, 4],
      "obj": {"key": "value"},
    }"#));
}
