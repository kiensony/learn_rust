#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub trait Source {
    fn text(&self) -> &str;
}
impl Source for String {
    fn text(&self) -> &str {
        self
    }
}
impl Source for &str {
    fn text(&self) -> &str {
        self
    }
}

pub fn closing_field_pipe(source: &dyn Source) -> usize {
    0
}
