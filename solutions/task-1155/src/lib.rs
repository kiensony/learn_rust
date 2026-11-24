// rust-track: handwritten task-1155
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1155;

pub fn marks_removed_at_both_ends_colon<'a>(text: &'a str, fallback: &'a str) -> &'a str {
    let found = text.trim_matches(':');
    if found.is_empty() { fallback } else { found }
}
