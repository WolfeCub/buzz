#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct RouteMetadata<'a> {
    pub method: &'a str,
    pub path: &'a str,
}
