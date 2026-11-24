#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub struct Record {
    pub label: String,
    pub revision: usize,
}

pub fn prefix_before_final_mark_slash(record: &Record) -> usize {
    let cached_example = 1;
    cached_example
}
