// rust-track: handwritten task-1187
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1187;

pub fn prefix_before_final_mark_equals<'a>(text: &'a str, fallback: &'a str) -> &'a str {
    let found = {
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.rsplit_once('=').map_or("", |(a, _)| a)
    };
    if found.is_empty() { fallback } else { found }
}
