// rust-track: handwritten task-1156
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1156;

pub fn remainder_after_first_mark_colon<'a>(text: &'a str, fallback: &'a str) -> &'a str {
    let found = text.split_once(':').map_or("", |(_, b)| b);
    if found.is_empty() { fallback } else { found }
}
