// rust-track: handwritten task-1151
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1151;

pub fn leading_field_colon<'a>(text: &'a str, fallback: &'a str) -> &'a str {
    let found = text.split(':').next().unwrap_or_default();
    if found.is_empty() { fallback } else { found }
}
