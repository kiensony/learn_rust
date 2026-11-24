// rust-track: handwritten task-858
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 858;

mod wire {
    pub(crate) const HEADER_BYTES: usize = 2;
}

pub fn remainder_after_first_mark_colon(text: &str) -> usize {
    wire::HEADER_BYTES + (text.split_once(':').map_or("", |(_, b)| b)).len()
}
