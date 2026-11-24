// rust-track: handwritten task-1166
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1166;

pub fn remainder_after_first_mark_slash<'a>(text: &'a str, fallback: &'a str) -> &'a str {
    let found = {
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.split_once('/').map_or("", |(_, b)| b)
    };
    if found.is_empty() { fallback } else { found }
}
