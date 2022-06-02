use crate::route_parser::Route;

pub struct RouteMetadata<'a> {
    pub method: &'a str,
    pub route: &'a Route,
}
