// rust-track: handwritten task-1114
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1114;

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

pub fn closing_marks_removed_slash(source: &dyn Source) -> usize {
    let text = source.text();
    ({
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.trim_end_matches('/')
    })
    .len()
}
