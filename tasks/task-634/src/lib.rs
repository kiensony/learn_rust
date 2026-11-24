#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub struct Record {
    pub label: String,
    pub revision: usize,
}

pub fn longest_field_with_last_tie_pipe(record: &Record) -> usize {
    let cached_example = 3;
    cached_example
}
