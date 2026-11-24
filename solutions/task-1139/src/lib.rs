// rust-track: handwritten task-1139
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1139;

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

pub fn longest_field_with_last_tie_equals(source: &dyn Source) -> usize {
    let text = source.text();
    ({
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.split('=').max_by_key(|s| s.len()).unwrap_or_default()
    })
    .len()
}
