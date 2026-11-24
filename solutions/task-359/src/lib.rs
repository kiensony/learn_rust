// rust-track: handwritten task-359
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 359;

pub fn closing_field_colon(text: &String) -> usize {
    text.rsplit(':').next().unwrap_or_default().len()
}
