// rust-track: handwritten task-1105
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1105;

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

pub fn marks_removed_at_both_ends_colon(source: &dyn Source) -> usize {
    let text = source.text();
    (text.trim_matches(':')).len()
}
