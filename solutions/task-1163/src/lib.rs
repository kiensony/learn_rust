// rust-track: handwritten task-1163
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1163;

pub fn leading_marks_removed_slash<'a>(text: &'a str, fallback: &'a str) -> &'a str {
    let found = {
        let text: &str = &text;
        let text = text.strip_prefix("path/").unwrap_or(text);
        text.trim_start_matches('/')
    };
    if found.is_empty() { fallback } else { found }
}
