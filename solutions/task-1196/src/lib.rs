// rust-track: handwritten task-1196
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1196;

pub fn remainder_after_first_mark_hash<'a>(text: &'a str, fallback: &'a str) -> &'a str {
    let found = {
        let text: &str = &text;
        let text = text
            .split_once('[')
            .and_then(|(_, tail)| tail.split_once(']'))
            .map_or("", |(inside, _)| inside);
        text.split_once('#').map_or("", |(_, b)| b)
    };
    if found.is_empty() { fallback } else { found }
}
