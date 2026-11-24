// rust-track: handwritten task-917
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 917;

pub fn marks_removed_at_both_ends_slash(text: &str, mut visit: impl FnMut(&str) -> usize) -> usize {
    visit({
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.trim_matches('/')
    })
}
