use std::fmt::Debug;

use crate::{HttpMethod, Handler};

/// Represents the type of a url segment.
///
/// For example the url `/foo/{varname}` would be broken down into `SegmentType::Const("foo")` and
/// `SegmentType::Variable("varname")`. Since the matching behaviour is different between the two
/// types it helps to have the differentiated.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum SegmentType {
    /// A static segment of a URL that will only match it's exact text
    Const(&'static str),
    /// A dynamic segment of the URL that can be passed in as a variable to a function
    Variable(&'static str),
    /// A placeholder for empty or terminal segments
    SegNone,
}

/// Represents a tree of url routes.
#[derive(Clone)]
pub struct Route {
    /// Denotes the type of this portion of the url.
    ///
    /// See [`SegmentType`] for more details.
    pub segment: SegmentType,
    /// Recursively contains routes that share this segment as a prefix.
    pub children: Vec<Route>,
    /// The function that should be called when this route is hit.
    ///
    /// This is [`Option`]al since not every part of the tree is routeable.
    /// Consider a server that has `/foo/bar` and `/foo/quox` as routes.
    /// `foo` would appear in the tree as a [`Route`] yet it's not valid to hit on it's own
    pub handler: Option<Handler>,
    /// If the route is valid to be hit (see [`Route::handler`] for more details) this determines
    /// what HTTP methods are valid for it.
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

    pub fn from_vec(flat: &[SegmentType], method: &HttpMethod, handler: Handler) -> Route {
        let mut root = Route::new();

        let mut cursor = &mut root;

        for i in 0..flat.len() {
            cursor.segment = flat[i];

            if i == flat.len() - 1 {
                cursor.method = Some(*method);
                cursor.handler = Some(handler);
            } else {
                let new = Route::new();
                cursor.children.push(new);
                cursor = &mut cursor.children[0];
            }
        }

        root
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

