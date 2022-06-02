#[derive(PartialEq, Eq, Debug)]
pub enum SegmentType {
    Const(&'static str),
    Variable(&'static str),
}

pub struct Route {
    pub path: &'static str,
    pub segments: &'static [SegmentType],
}
