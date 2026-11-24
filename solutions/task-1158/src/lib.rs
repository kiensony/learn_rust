// rust-track: handwritten task-1158
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1158;

pub fn first_nonempty_field_colon<'a>(text: &'a str, fallback: &'a str) -> &'a str {
    let found = text.split(':').find(|s| !s.is_empty()).unwrap_or_default();
    if found.is_empty() { fallback } else { found }
}
