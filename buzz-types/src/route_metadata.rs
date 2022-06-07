use crate::{HttpMethod, SegmentType};

pub struct RouteMetadata {
    pub route: &'static [SegmentType],
    pub method: HttpMethod,
}
