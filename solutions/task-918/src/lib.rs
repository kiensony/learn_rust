// rust-track: handwritten task-918
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 918;

pub fn remainder_after_first_mark_slash(text: &str, mut visit: impl FnMut(&str) -> usize) -> usize {
    visit({
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.split_once('/').map_or("", |(_, b)| b)
    })
}
