use std::error::Error;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

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

    pub fn route(
        mut self,
        tuple: (fn(&HttpRequest, Vec<&str>) -> HttpResponse, RouteMetadata),
    ) -> Self {
        fn recurse(
            segments: &[SegmentType],
            routes: &mut Vec<Route>,
            method: &HttpMethod,
            handler: fn(&HttpRequest, Vec<&str>) -> HttpResponse,
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
        dbg!(&self.routes);
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

        if let Some(response) = match_route_params(request, &self.routes) {
            write_response(&mut stream, &response)?;
            return Ok(());
        }

        write_response(&mut stream, &HttpResponse::new(HttpStatusCode::NotFound))?;

        Ok(())
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
    let segments: Vec<_> = request.path.split("/").filter(|p| !p.is_empty()).collect();

    let candidates = route_tree_filter(&segments, &routes, request.method);
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

    Some(handler(&request, vars))
}

fn route_tree_filter<'a>(segments: &[&str], routes: &[Route], method: HttpMethod) -> Vec<Route> {
    if segments.len() <= 0 {
        return Vec::new();
    }

    let routes: Vec<_> = routes
        .iter()
        .filter(|r| match r.segment {
            SegmentType::Const(text) => *text == *segments[0] && r.method == Some(method),
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

fn find_most_specific(routes: &[Route]) -> (fn(&HttpRequest, Vec<&str>) -> HttpResponse, Route) {
    fn helper(
        routes: &[Route],
        depth: usize,
    ) -> (usize, fn(&HttpRequest, Vec<&str>) -> HttpResponse, Route) {
        routes
            .iter()
            .map(|r| {
                if r.children.len() > 0 {
                    let (depth, handler, route) = helper(&r.children, depth + 1);
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
                } else {
                    (
                        depth,
                        r.handler
                            .expect("Somehow we're at a leaf that has no handler"),
                        r.clone(),
                    )
                }
            })
            .max_by_key(|r| r.0)
            .unwrap()
    }

    let (depth, handler, route) = helper(routes, 0);
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

pub fn vec_to_route(
    flat: &[SegmentType],
    method: &HttpMethod,
    handler: fn(&HttpRequest, Vec<&str>) -> HttpResponse,
) -> Route {
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
