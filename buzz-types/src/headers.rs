#[derive(Debug)]
pub struct Headers<'a> {
    pub content_type: Option<&'a str>,
    pub content_length: Option<&'a str>,
    pub remaining: Vec<(&'a str, &'a str)>,
}

impl<'a> Headers<'a> {
    fn with_capacity(len: usize) -> Self {
        Self {
            content_type: None,
            content_length: None,
            remaining: Vec::with_capacity(len)
        }
    }

    pub fn from_iter(pairs: Vec<(&'a str, &'a str)>) -> Self
    {
        let mut result = Headers::with_capacity(pairs.len());

        for (key, val) in pairs {
            match key {
                "Content-Length" => {
                    result.content_length = Some(val);
                }
                "Content-Type" => {
                    result.content_type = Some(val);
                }
                _ => {
                    result.remaining.push((key, val));
                }
            };
        }

        result
    }

    pub fn get(&self, key: &'a str) -> Option<&'a str> {
        self.remaining
            .iter()
            .copied()
            .find_map(|(k, v)| if k == key { Some(v) } else { None })
    }
}
