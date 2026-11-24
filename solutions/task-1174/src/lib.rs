// rust-track: handwritten task-1174
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1174;

pub fn closing_marks_removed_pipe<'a>(text: &'a str, fallback: &'a str) -> &'a str {
    let found = {
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.trim_end_matches('|')
    };
    if found.is_empty() { fallback } else { found }
}
