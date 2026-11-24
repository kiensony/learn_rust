// rust-track: handwritten task-1104
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1104;

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

pub fn closing_marks_removed_colon(source: &dyn Source) -> usize {
    let text = source.text();
    (text.trim_end_matches(':')).len()
}
