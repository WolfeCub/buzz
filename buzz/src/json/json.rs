use std::ops::Deref;

use buzz_types::{errors::JsonError, traits::{Deserialize, FromBody}};

use super::JsonValue;

pub struct Json<T> {
    inner: T,
}

impl<T: Deserialize<JsonValue>> FromBody for Json<T> {
    type Err = JsonError;

    fn from_body(body: &str) -> Result<Self, JsonError> {
        let value = JsonValue::parse(body)?;

        Ok(Self {
            inner: T::deserialize(value)?,
        })
    }
}

impl<T> Json<T> {
    pub fn get(&self) -> &T {
        &self.inner
    }
}

impl<T> Deref for Json<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
