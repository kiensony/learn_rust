// rust-track: handwritten task-908
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 908;

pub fn remainder_after_first_mark_colon(text: &str, mut visit: impl FnMut(&str) -> usize) -> usize {
    visit(text.split_once(':').map_or("", |(_, b)| b))
}
