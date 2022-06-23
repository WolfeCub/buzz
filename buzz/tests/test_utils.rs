use proptest::prelude::*;

#[macro_export]
macro_rules! request {
    ($method:tt, $path:literal) => {
        request!($method, $path.to_owned(), , None)
    };
    ($method:tt, $path:expr) => {
        request!($method, $path, , None)
    };
    ($method:tt, $path:literal, $($key:literal: $value:expr),*) => {
        request!($method, $path.to_owned(), $($key: $value),*, None)
    };
    ($method:tt, $path:expr, $($key:literal: $value:expr),*) => {
        request!($method, $path, $($key: $value),*, None)
    };
    ($method:tt, $path:literal, $body:literal) => {
        request!($method, $path.to_owned(), , Some($body.to_owned()))
    };
    ($method:tt, $path:literal, $body:expr) => {
        request!($method, $path.to_owned(), , Some($body))
    };
    ($method:tt, $path:expr, $($key:literal: $value:expr),*, $body:expr) => {
        CONTEXT.get_or_init(make_buzz).dispatch(HttpRequest {
            method: HttpMethod::$method,
            path: $path,
            version: 1.1,
            headers: HashMap::from_iter([$(($key.to_owned(), $value)),*]),
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
