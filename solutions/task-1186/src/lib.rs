// rust-track: handwritten task-1186
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1186;

pub fn remainder_after_first_mark_equals<'a>(text: &'a str, fallback: &'a str) -> &'a str {
    let found = {
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.split_once('=').map_or("", |(_, b)| b)
    };
    if found.is_empty() { fallback } else { found }
}
