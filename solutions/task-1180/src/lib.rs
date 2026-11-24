// rust-track: handwritten task-1180
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1180;

pub fn shortest_field_with_first_tie_pipe<'a>(text: &'a str, fallback: &'a str) -> &'a str {
    let found = {
        let text: &str = &text;
        let text = text.lines().last().unwrap_or_default();
        text.split('|').min_by_key(|s| s.len()).unwrap_or_default()
    };
    if found.is_empty() { fallback } else { found }
}
