// rust-track: handwritten task-1162
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1162;

pub fn closing_field_slash<'a>(text: &'a str, fallback: &'a str) -> &'a str {
    let found = {
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.rsplit('/').next().unwrap_or_default()
    };
    if found.is_empty() { fallback } else { found }
}
