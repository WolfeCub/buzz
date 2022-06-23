use crate::errors::DeserializationError;

pub trait Deserialize<T>
where
    Self: Sized,
{
    fn deserialize(val: T) -> Result<Self, DeserializationError>;
}
