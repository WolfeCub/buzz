use proptest::prelude::*;

#[macro_export]
macro_rules! request {
    ($buzz:expr, $method:tt, $path:literal) => {
        request!($buzz, $method, $path, , None)
    };
    ($method:tt, $path:literal) => {
        request!($method, $path, , None)
    };
    ($method:tt, $path:expr) => {
        request!($method, &$path, , None)
    };
    ($method:tt, $path:literal, $($key:literal: $value:expr),*) => {
        request!($method, $path, $($key: $value),*, None)
    };
    ($method:tt, $path:expr, $($key:literal: $value:expr),*) => {
        request!($method, &$path, $($key: $value),*, None)
    };
    ($method:tt, $path:literal, body: $body:literal) => {
        request!($method, $path, , Some($body.to_owned()))
    };
    ($method:tt, $path:literal, body: $body:expr) => {
        request!($method, $path, , Some($body))
    };
    ($method:tt, $path:expr, $($key:literal: $value:expr),*, $body:expr) => {
        request!(make_buzz(), $method, $path, $($key: $value),*, $body)
    };
    ($buzz:expr, $method:tt, $path:expr, $($key:literal: $value:expr),*, $body:expr) => {
        $buzz.dispatch(HttpRequest {
            method: HttpMethod::$method,
            path: &$path,
            version: 1.1,
            headers: HashMap::from_iter([$(($key, $value.as_str())),*]),
            body: $body,
        })
    };
}

pub(crate) fn valid_str() -> impl Strategy<Value = String> {
    "[^\\\\\"]*"
}

pub(crate) fn valid_str_vec() -> impl Strategy<Value = Vec<String>> {
    prop::collection::vec(valid_str(), 0..10)
}
