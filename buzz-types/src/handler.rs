use std::collections::HashMap;

use crate::{HttpResponse, BuzzContext};

pub type Handler = fn(Vec<&str>, HashMap<&str, &str>, BuzzContext) -> HttpResponse;
