pub trait FromBody
where
    Self: Sized,
{
    type Err;

    fn from_body(body: &str) -> Result<Self, Self::Err>;
}
