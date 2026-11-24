// rust-track: handwritten task-522
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 522;

pub fn marks_removed_at_both_ends_slash(text: &str) -> usize {
    ({
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.trim_matches('/')
    })
    .chars()
    .count()
}
