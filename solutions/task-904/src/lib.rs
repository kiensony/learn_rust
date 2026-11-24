// rust-track: handwritten task-904
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 904;

pub fn closing_field_colon(text: &str, mut visit: impl FnMut(&str) -> usize) -> usize {
    visit(text.rsplit(':').next().unwrap_or_default())
}
