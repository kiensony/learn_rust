// rust-track: handwritten task-520
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 520;

pub fn leading_marks_removed_slash(text: &str) -> usize {
    ({
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.trim_start_matches('/')
    })
    .chars()
    .count()
}
