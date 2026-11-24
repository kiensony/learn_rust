// rust-track: handwritten task-916
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 916;

pub fn closing_marks_removed_slash(text: &str, mut visit: impl FnMut(&str) -> usize) -> usize {
    visit({
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.trim_end_matches('/')
    })
}
