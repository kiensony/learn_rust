// rust-track: handwritten task-1171
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1171;

pub fn leading_field_pipe<'a>(text: &'a str, fallback: &'a str) -> &'a str {
    let found = {
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.split('|').next().unwrap_or_default()
    };
    if found.is_empty() { fallback } else { found }
}
