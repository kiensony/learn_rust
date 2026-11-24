// rust-track: handwritten task-363
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 363;

pub fn remainder_after_first_mark_colon(text: &String) -> usize {
    text.split_once(':').map_or("", |(_, b)| b).len()
}
