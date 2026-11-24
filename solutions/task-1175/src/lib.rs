// rust-track: handwritten task-1175
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1175;

pub fn marks_removed_at_both_ends_pipe<'a>(text: &'a str, fallback: &'a str) -> &'a str {
    let found = {
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.trim_matches('|')
    };
    if found.is_empty() { fallback } else { found }
}
