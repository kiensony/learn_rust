// rust-track: handwritten task-1152
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1152;

pub fn closing_field_colon<'a>(text: &'a str, fallback: &'a str) -> &'a str {
    let found = text.rsplit(':').next().unwrap_or_default();
    if found.is_empty() { fallback } else { found }
}
