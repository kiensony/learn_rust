// rust-track: handwritten task-521
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 521;

pub fn closing_marks_removed_slash(text: &str) -> usize {
    ({
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.trim_end_matches('/')
    })
    .chars()
    .count()
}
