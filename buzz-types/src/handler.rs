use std::collections::HashMap;

use crate::{HttpRequest, HttpResponse};

pub type Handler = fn(&HttpRequest, Vec<&str>, HashMap<&str, &str>) -> HttpResponse;
