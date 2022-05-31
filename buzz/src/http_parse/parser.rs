#[derive(Debug)]
pub struct Parser<'a> {
    pub data: &'a [u8],
    pub offset: usize,
}

impl<'a> Parser<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, offset: 0 }
    }

    pub fn peek(&self) -> Option<u8> {
        if self.offset >= self.data.len() {
            None
        } else {
            let r = self.data[self.offset];
            Some(r)
        }
    }

    pub fn consume(&mut self, n: usize) {
        self.offset += n;
    }

    pub fn consume_while(&mut self, predicate: fn(u8) -> bool) {
        while let Some(_) = self.take_if(predicate) {}
    }

    pub fn take(&mut self) -> Option<u8> {
        let r = self.peek();
        if r.is_some() {
            self.offset += 1;
        }
        r
    }

    pub fn take_n(&mut self, n: usize) -> Option<&[u8]> {
        if self.offset + n <= self.data.len() {
            let r = Some(&self.data[self.offset..self.offset + n]);
            self.offset += n;
            r
        } else {
            None
        }
    }

    pub fn take_if(&mut self, predicate: fn(u8) -> bool) -> Option<u8> {
        let r = self.peek();
        if r.is_some() && r.map(predicate).unwrap() {
            self.offset += 1;
            r
        } else {
            None
        }
    }

    pub fn substr_to_offset(&self, starting: usize) -> &str {
        self.substr(starting, self.offset)
    }

    pub fn substr(&self, starting: usize, ending: usize) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.data[starting..ending]) }
    }
}
