// rust-track: handwritten task-1164
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1164;

pub fn closing_marks_removed_slash<'a>(text: &'a str, fallback: &'a str) -> &'a str {
    let found = {
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.trim_end_matches('/')
    };
    if found.is_empty() { fallback } else { found }
}
