// rust-track: handwritten task-1106
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1106;

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

pub fn remainder_after_first_mark_colon(source: &dyn Source) -> usize {
    let text = source.text();
    (text.split_once(':').map_or("", |(_, b)| b)).len()
}
