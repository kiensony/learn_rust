// rust-track: handwritten task-1188
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1188;

pub fn first_nonempty_field_equals<'a>(text: &'a str, fallback: &'a str) -> &'a str {
    let found = {
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.split('=').find(|s| !s.is_empty()).unwrap_or_default()
    };
    if found.is_empty() { fallback } else { found }
}
