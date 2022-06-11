use std::collections::HashMap;
use std::error::Error;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::ops::Index;

use crate::http_parse::*;
use buzz_types::*;

pub struct Buzz {
    addr: &'static str,
    routes: Vec<Route>,
}

impl Buzz {
    pub fn new(addr: &'static str) -> Self {
        Self {
            addr,
            routes: Vec::new(),
        }
    }

    pub fn route(mut self, tuple: (Handler, RouteMetadata)) -> Self {
        fn recurse(
            segments: &[SegmentType],
            routes: &mut Vec<Route>,
            method: &HttpMethod,
            handler: Handler,
        ) {
            if segments.len() == 0 {
                return;
            }

            /* TODO: Lots 'o repetition. Refactor this */
            match segments[0] {
                SegmentType::Const(text) => {
                    if let Some(route) = routes.iter_mut().find(|r| match r.segment {
                        SegmentType::Const(route_text) => {
                            *text == *route_text && r.method == Some(*method)
                        }
                        _ => false,
                    }) {
                        recurse(&segments[1..], &mut route.children, method, handler);
                    } else {
                        routes.push(vec_to_route(segments, method, handler));
                    }
                }
                SegmentType::Variable(var_name) => {
                    if let Some(route) = routes.iter_mut().find(|r| match r.segment {
                        SegmentType::Variable(route_var_name) => *var_name == *route_var_name,
                        _ => false,
                    }) {
                        recurse(&segments[1..], &mut route.children, method, handler);
                    } else {
                        routes.push(vec_to_route(segments, method, handler));
                    }
                }
                SegmentType::SegNone => {
                    routes.push(vec_to_route(segments, method, handler));
                }
            }
        }

        recurse(tuple.1.route, &mut self.routes, &tuple.1.method, tuple.0);

        self
    }

    pub fn run_server(&self) {
        let listener = TcpListener::bind(self.addr).unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            match self.handle_connection(stream) {
                Ok(_) => {}
                Err(e) => panic!("{}", e),
            }
        }
    }

    fn handle_connection(&self, mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
        let mut buffer = [0; 1024];

        stream.read(&mut buffer)?;

        let request = parse_http(&buffer)?;
        let response = self.dispatch(request);

        write_response(&mut stream, &response)?;

        Ok(())
    }

    pub fn dispatch(&self, request: HttpRequest) -> HttpResponse {
        match_route_params(request, &self.routes)
            .unwrap_or(HttpResponse::new(HttpStatusCode::NotFound))
    }
}

fn write_response(stream: &mut TcpStream, request: &HttpResponse) -> std::io::Result<()> {
    /* TODO: Not hardcoded version. What do we actually support? */
    stream.write(b"HTTP/1.1 ")?;
    stream.write((request.status_code as u32).to_string().as_bytes())?;
    stream.write(b" ")?;
    stream.write(request.status_code.to_string().as_bytes())?;
    stream.write(b"\r\n")?;

    for (key, value) in &request.headers {
        stream.write(key.as_bytes())?;
        stream.write(b": ")?;
        stream.write(value.as_bytes())?;
        stream.write(b"\r\n")?;
    }

    stream.write(b"\r\n")?;

    /* TODO: Buffer? */
    if let Some(body) = &request.body {
        stream.write(body.as_bytes())?;
    }
    stream.flush()?;

    stream.shutdown(std::net::Shutdown::Both)?;
    Ok(())
}

fn match_route_params<'a>(request: HttpRequest, routes: &Vec<Route>) -> Option<HttpResponse> {
    let query_seperator = request.path.chars().position(|c| c == '?');
    let route_path = &request.path[0..(query_seperator.unwrap_or(request.path.len()))];
    let segments: Vec<_> = route_path.split("/").filter(|p| !p.is_empty()).collect();

    let query_params = if let Some(index) = query_seperator {
        parse_query_params(&request.path[index + 1..])
    } else {
        HashMap::new()
    };

    let candidates = route_tree_filter(&segments, &routes, request.method);

    if candidates.len() == 0 {
        return None;
    };
    let (handler, route) = find_most_specific(&candidates);
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

    Some(handler(vars, query_params, context))
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

fn find_most_specific(routes: &[Route]) -> (Handler, Route) {
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

    /* TODO: Maybe don't assume this is the case so this helper can be used elsewhere? */
    let (_, handler, route) = helper(routes, 0).expect(
        "There's at least one route that exists since we check routes is non empty",
    );
    (handler, route)
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

pub fn vec_to_route(flat: &[SegmentType], method: &HttpMethod, handler: Handler) -> Route {
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

pub fn parse_query_params(query_params: &str) -> HashMap<&str, &str> {
    HashMap::from_iter(query_params.split("&").filter_map(|kvp| {
        if kvp.is_empty() {
            return None;
        }

        let index = kvp.chars().position(|c| c == '=');

        index.map(|i| (&kvp[0..i], &kvp[i + 1..]))
    }))
}
