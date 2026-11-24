// rust-track: handwritten task-915
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 915;

pub fn leading_marks_removed_slash(text: &str, mut visit: impl FnMut(&str) -> usize) -> usize {
    visit({
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.trim_start_matches('/')
    })
}
