use std::ops::Deref;

use buzz_types::traits::{FromBody, Deserialize};

use super::JsonValue;

pub struct Json<T> {
    inner: T,
}

impl<T: Deserialize<JsonValue>> FromBody for Json<T> {
    fn from_body(body: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let value = JsonValue::parse(body)?;

        Ok(Self {
            inner: T::deserialize(value)?,
        })
    }
}

impl<T> Json<T> {
    fn get(&self) -> &T {
        &self.inner
    }
}

impl<T> Deref for Json<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
