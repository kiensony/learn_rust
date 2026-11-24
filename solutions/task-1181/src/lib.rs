// rust-track: handwritten task-1181
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1181;

pub fn leading_field_equals<'a>(text: &'a str, fallback: &'a str) -> &'a str {
    let found = {
        let text: &str = &text;
        let text = text.split_once(';').map_or(text, |(data, _)| data);
        text.split('=').next().unwrap_or_default()
    };
    if found.is_empty() { fallback } else { found }
}
