#[derive(Debug)]
pub struct Headers<'a> {
    pub content_type: Option<&'a str>,
    pub content_length: Option<&'a str>,
    pub remaining: Vec<(&'a str, &'a str)>,
}

impl<'a> Headers<'a> {
    pub fn from_iter(pairs: Vec<(&'a str, &'a str)>) -> Self
    {
        let mut other_headers: Vec<(&str, &str)> = Vec::new();
        let mut content_length = None;
        let mut content_type = None;

        for (key, val) in pairs {
            match key {
                "Content-Length" => {
                    content_length = Some(val);
                }
                "Content-Type" => {
                    content_type = Some(val);
                }
                _ => {
                    other_headers.push((key, val));
                }
            };
        }

        Headers {
            content_type,
            content_length,
            remaining: other_headers,
        }
    }

    pub fn get(&self, key: &'a str) -> Option<&'a str> {
        self.remaining
            .iter()
            .copied()
            .find_map(|(k, v)| if k == key { Some(v) } else { None })
    }
}
