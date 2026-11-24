// rust-track: handwritten task-903
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 903;

pub fn leading_field_colon(text: &str, mut visit: impl FnMut(&str) -> usize) -> usize {
    visit(text.split(':').next().unwrap_or_default())
}
