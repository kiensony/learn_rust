// rust-track: handwritten task-1110
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1110;

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

pub fn shortest_field_with_first_tie_colon(source: &dyn Source) -> usize {
    let text = source.text();
    (text.split(':').min_by_key(|s| s.len()).unwrap_or_default()).len()
}
