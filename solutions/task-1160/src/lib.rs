// rust-track: handwritten task-1160
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1160;

pub fn shortest_field_with_first_tie_colon<'a>(text: &'a str, fallback: &'a str) -> &'a str {
    let found = text.split(':').min_by_key(|s| s.len()).unwrap_or_default();
    if found.is_empty() { fallback } else { found }
}
