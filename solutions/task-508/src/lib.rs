// rust-track: handwritten task-508
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 508;

pub fn leading_field_colon(text: &str) -> usize {
    (text.split(':').next().unwrap_or_default()).chars().count()
}
