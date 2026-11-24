// rust-track: handwritten task-1108
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1108;

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

pub fn first_nonempty_field_colon(source: &dyn Source) -> usize {
    let text = source.text();
    (text.split(':').find(|s| !s.is_empty()).unwrap_or_default()).len()
}
