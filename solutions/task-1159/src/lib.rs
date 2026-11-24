// rust-track: handwritten task-1159
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1159;

pub fn longest_field_with_last_tie_colon<'a>(text: &'a str, fallback: &'a str) -> &'a str {
    let found = text.split(':').max_by_key(|s| s.len()).unwrap_or_default();
    if found.is_empty() { fallback } else { found }
}
