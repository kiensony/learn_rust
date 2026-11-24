// rust-track: handwritten task-1129
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1129;

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

pub fn longest_field_with_last_tie_pipe(source: &dyn Source) -> usize {
    longest_field_with_last_tie_pipe_impl(source)
}

fn longest_field_with_last_tie_pipe_impl(source: &dyn Source) -> usize {
    let text = source.text();
    ({
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.split('|').max_by_key(|s| s.len()).unwrap_or_default()
    })
    .len()
}
