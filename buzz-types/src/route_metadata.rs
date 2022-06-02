use crate::Route;

pub struct RouteMetadata<'a> {
    pub method: &'a str,
    pub route: Route,
}
