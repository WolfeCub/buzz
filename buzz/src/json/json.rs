use std::marker::PhantomData;

use buzz_types::traits::FromBody;

use super::JsonValue;

pub struct Json<T> {
    inner: JsonValue,
    _t: PhantomData<T>
}

impl<T> FromBody for Json<T> {
    fn from_body(body: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            inner: JsonValue::parse(body)?,
            _t: PhantomData
        })
    }
}
