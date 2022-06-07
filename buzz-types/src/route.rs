use std::fmt::Debug;

use crate::{HttpMethod, HttpRequest, HttpResponse};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum SegmentType {
    Const(&'static str),
    Variable(&'static str),
    SegNone,
}

#[derive(Clone)]
pub struct Route {
    pub segment: SegmentType,
    pub children: Vec<Route>,
    pub handler: Option<fn(&HttpRequest, Vec<&str>) -> HttpResponse>,
    pub method: Option<HttpMethod>,
}

impl Route {
    pub fn new() -> Self {
        Self {
            segment: SegmentType::SegNone,
            children: Vec::new(),
            handler: None,
            method: None,
        }
    }
}

impl Debug for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Route")
            .field("segment", &self.segment)
            .field("children", &self.children)
            .field("method", &self.method)
            .field("handler", &self.handler.is_some())
            .finish()
    }
}
