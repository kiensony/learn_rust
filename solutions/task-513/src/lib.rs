// rust-track: handwritten task-513
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 513;

pub fn remainder_after_first_mark_colon(text: &str) -> usize {
    (text.split_once(':').map_or("", |(_, b)| b))
        .chars()
        .count()
}
