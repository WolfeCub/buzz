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
    /* TODO: Not sure how fast this function is. Both algorithmicly and implementation wise. */
    pub fn match_route_params(&self, seg_types: &[SegmentType], method: HttpMethod) -> Option<HashMap<&str, &str>> {
        if method != self.method {
            return None;
        }

        let segments: Vec<_> = self.path.split("/").filter(|p| !p.is_empty()).collect();

        if segments.len() != seg_types.len() {
            return None;
        }

        let mut map = HashMap::new();

        /* TODO: This is pretty gnarly. Make it nicer. iter() without a reference? */
        for (seg, ty) in segments.iter().zip(seg_types) {
            let failed = match ty {
                SegmentType::Const(const_value) => {
                    if **const_value != **seg {
                        None
                    } else {
                        Some(())
                    }
                },
                SegmentType::Variable(var_name) => {
                    map.insert(*var_name, *seg);
                    Some(())
                },
            };

            if failed.is_none() {
                return None;
            }
        };

        Some(map)
    }
}
