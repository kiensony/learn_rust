// rust-track: handwritten task-358
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 358;

pub fn leading_field_colon(text: &String) -> usize {
    text.split(':').next().unwrap_or_default().len()
}
