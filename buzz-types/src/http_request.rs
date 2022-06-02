use std::collections::HashMap;

use crate::{HttpMethod, SegmentType};

#[derive(Debug)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: String,
    pub version: f64,
    pub headers: HashMap<String, String>,
}

impl HttpRequest {
    pub fn match_route_param(&self, seg_types: Vec<SegmentType>, name: &str) -> Option<String> {
        let segments: Vec<_> = self.path.split("/").collect();

        if segments.len() != seg_types.len() {
            return None;
        }

        segments.iter().zip(seg_types).find_map(|(seg, ty)| {
            match ty {
                SegmentType::Variable(var_name) => {
                    if *var_name == *name {
                        Some((*seg).to_owned())
                    } else {
                        None
                    }
                },
                _ => None
            }
        })
    }
}
