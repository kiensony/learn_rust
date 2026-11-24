// rust-track: handwritten task-1185
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1185;

pub fn marks_removed_at_both_ends_equals<'a>(text: &'a str, fallback: &'a str) -> &'a str {
    let found = {
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.trim_matches('=')
    };
    if found.is_empty() { fallback } else { found }
}
