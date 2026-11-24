#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub struct Record {
    pub label: String,
    pub revision: usize,
}

pub fn remainder_after_first_mark_slash(record: &Record) -> usize {
    0
}
