mod http_parse;
mod buzz;

pub mod types;

pub mod dev {
    pub use linkme;
}

pub mod prelude {
    use linkme;
    use super::types::HttpService;

    pub use super::buzz::*;

    #[linkme::distributed_slice]
    pub static BUZZ_REGISTRY: [HttpService] = [..];
}
