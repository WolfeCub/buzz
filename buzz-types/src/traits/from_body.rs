use std::error::Error;

pub trait FromBody
where
    Self: Sized,
{
    fn from_body(body: &str) -> Result<Self, Box<dyn Error>>;
}
