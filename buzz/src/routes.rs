use buzz_types::{dev::DependancyInjection, errors::BuzzError, *};
use std::collections::HashMap;

pub struct Routes {
    routes: Vec<Route>,
}

impl Routes {
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }

    pub fn match_route_params(
        &self,
        request: HttpRequest,
        di: &DependancyInjection,
    ) -> Result<HttpResponse, BuzzError> {
        let query_seperator = request.path.chars().position(|c| c == '?');
        let route_path = &request.path[0..(query_seperator.unwrap_or(request.path.len()))];
        let segments: Vec<_> = route_path.split("/").filter(|p| !p.is_empty()).collect();

        let query_params = if let Some(index) = query_seperator {
            parse_query_params(&request.path[index + 1..])
        } else {
            HashMap::new()
        };

        let candidates = route_tree_filter(&segments, &self.routes, request.method);
        if let Some((handler, route)) = find_most_specific(&candidates) {
            let flat = unsafe_flatten(&route);

            let vars = flat
                .iter()
                .zip(segments)
                .filter_map(|(ty, val)| {
                    if let SegmentType::Variable(_) = ty {
                        Some(val)
                    } else {
                        None
                    }
                })
                .collect();

            let context = BuzzContext {
                headers: request.headers,
            };

            handler(vars, query_params, request.body, context, di)
        } else {
            Ok(HttpResponse::new(HttpStatusCode::NotFound))
        }
    }

    pub fn insert(&mut self, routes: Vec<(Handler, RouteMetadata)>) {
        fn recurse(
            segments: &[SegmentType],
            routes: &mut Vec<Route>,
            method: &HttpMethod,
            handler: Handler,
        ) {
            if segments.len() == 0 {
                return;
            }

            if !routes.iter_mut().any(|route| {
                /* If the verbatim text matches or the variable is the right name then we can
                 * look deeper down that route since it could potentially still match.
                 */
                if match (segments[0], route.segment) {
                    (SegmentType::Const(text), SegmentType::Const(route_text))
                        if *text == *route_text && route.method == Some(*method) =>
                    {
                        true
                    }

                    (SegmentType::Variable(var_name), SegmentType::Variable(route_var_name))
                        if *var_name == *route_var_name =>
                    {
                        true
                    }
                    _ => false,
                } {
                    recurse(&segments[1..], &mut route.children, method, handler);
                    true
                } else {
                    false
                }
            }) {
                /* If we don't match anything then this is the point where the route is new so we add it */
                routes.push(Route::from_vec(segments, method, handler));
            }
        }

        for (handler, metadata) in routes {
            recurse(metadata.route, &mut self.routes, &metadata.method, handler);
        }
    }
}

fn route_tree_filter<'a>(segments: &[&str], routes: &[Route], method: HttpMethod) -> Vec<Route> {
    if segments.len() <= 0 {
        return Vec::new();
    }

    let routes: Vec<_> = routes
        .iter()
        .filter(|r| match r.segment {
            SegmentType::Const(text) => {
                let text_matches = *text == *segments[0];
                /* If this is the leaf then we should care about method */
                if segments.len() == 1 {
                    text_matches && r.method == Some(method)
                } else {
                    text_matches
                }
            }
            SegmentType::Variable(_) => true,
            SegmentType::SegNone => false,
        })
        .collect();

    routes
        .iter()
        .map(|r| Route {
            segment: r.segment,
            children: route_tree_filter(&segments[1..], &r.children, method),
            handler: r.handler,
            method: r.method,
        })
        .collect()
}

fn find_most_specific(routes: &[Route]) -> Option<(Handler, Route)> {
    fn helper(routes: &[Route], depth: usize) -> Option<(usize, Handler, Route)> {
        routes
            .iter()
            .filter_map(|r| {
                if r.children.len() > 0 {
                    helper(&r.children, depth + 1).map(|(depth, handler, route)| {
                        (
                            depth,
                            handler,
                            Route {
                                segment: r.segment,
                                children: vec![route],
                                handler: r.handler,
                                method: r.method,
                            },
                        )
                    })
                } else if r.handler.is_some() {
                    Some((depth, r.handler.unwrap(), r.clone()))
                } else {
                    None
                }
            })
            .max_by_key(|r| r.0)
    }

    helper(routes, 0).map(|(_, handler, route)| (handler, route))
}

fn unsafe_flatten(route: &Route) -> Vec<SegmentType> {
    let mut cursor = route;

    let mut acc = Vec::new();

    loop {
        match cursor.segment {
            SegmentType::SegNone => break,
            otherwise => acc.push(otherwise),
        }

        if cursor.children.len() > 0 {
            cursor = &cursor.children[0];
        } else {
            break;
        }
    }

    acc
}

pub fn parse_query_params(query_params: &str) -> HashMap<&str, &str> {
    HashMap::from_iter(query_params.split("&").filter_map(|kvp| {
        if kvp.is_empty() {
            return None;
        }

        let index = kvp.chars().position(|c| c == '=');

        index.map(|i| (&kvp[0..i], &kvp[i + 1..]))
    }))
}
