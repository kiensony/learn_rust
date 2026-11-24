// rust-track: handwritten task-1134
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1134;

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

pub fn closing_marks_removed_equals(source: &dyn Source) -> usize {
    closing_marks_removed_equals_impl(source)
}

fn closing_marks_removed_equals_impl(source: &dyn Source) -> usize {
    let text = source.text();
    ({
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.trim_end_matches('=')
    })
    .len()
}
