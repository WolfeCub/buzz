use std::cell::Cell;

#[derive(Debug)]
pub struct Parser<'a> {
    pub data: &'a [u8],
    offset: Cell<usize>,
}

impl<'a> Parser<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, offset: Cell::new(0) }
    }

    pub fn offset(&self) -> usize {
        self.offset.get()
    }

    pub fn peek(&self) -> Option<u8> {
        let offset = self.offset.get();
        if offset >= self.data.len() {
            None
        } else {
            let r = self.data[offset];
            Some(r)
        }
    }

    pub fn consume(&self, n: usize) {
        let value = self.offset.get();
        self.offset.set(value + n);
    }

    pub fn consume_while(&self, predicate: fn(u8) -> bool) {
        while let Some(_) = self.take_if(predicate) {}
    }

    pub fn take(&self) -> Option<u8> {
        let r = self.peek();
        if r.is_some() {
            let val = self.offset.get();
            self.offset.set(val + 1);
        }
        r
    }

    pub fn take_n(&self, n: usize) -> Option<&[u8]> {
        let offset = self.offset.get();
        if offset + n <= self.data.len() {
            let r = Some(&self.data[offset..offset + n]);
            self.offset.set(offset + n);
            r
        } else {
            None
        }
    }

    pub fn take_if(&self, predicate: fn(u8) -> bool) -> Option<u8> {
        let r = self.peek();
        if r.is_some() && r.map(predicate).unwrap() {
            let val = self.offset.get();
            self.offset.set(val + 1);
            r
        } else {
            None
        }
    }

    pub fn substr_to_offset(&self, starting: usize) -> &'a str {
        self.substr(starting, self.offset.get())
    }

    pub fn substr(&self, starting: usize, ending: usize) -> &'a str {
        unsafe { std::str::from_utf8_unchecked(&self.data[starting..ending]) }
    }

    pub fn subbytes(&self, starting: usize, ending: usize) -> &'a [u8] {
        &self.data[starting..ending]
    }

    pub fn subbytes_to_offset(&self, starting: usize) -> &'a [u8] {
        self.subbytes(starting, self.offset.get())
    }

    pub fn remaining(&self) -> usize {
        self.data.len() - self.offset.get()
    }

    pub fn rewind(&self, n: usize) {
        let offset = self.offset.get();
        self.offset.set(offset - n);
    }
}
