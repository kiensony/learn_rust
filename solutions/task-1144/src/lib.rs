// rust-track: handwritten task-1144
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1144;

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

pub fn closing_marks_removed_hash(source: &dyn Source) -> usize {
    closing_marks_removed_hash_impl(source)
}

fn closing_marks_removed_hash_impl(source: &dyn Source) -> usize {
    let text = source.text();
    ({
        let text: &str = &text;
        let text = text
            .split_once('[')
            .and_then(|(_, tail)| tail.split_once(']'))
            .map_or("", |(inside, _)| inside);
        text.trim_end_matches('#')
    })
    .len()
}
