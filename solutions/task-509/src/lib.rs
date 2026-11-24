// rust-track: handwritten task-509
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 509;

pub fn closing_field_colon(text: &str) -> usize {
    (text.rsplit(':').next().unwrap_or_default())
        .chars()
        .count()
}
