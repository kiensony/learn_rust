// rust-track: handwritten task-1123
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1123;

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

pub fn leading_marks_removed_pipe(source: &dyn Source) -> usize {
    let text = source.text();
    ({
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.trim_start_matches('|')
    })
    .len()
}
