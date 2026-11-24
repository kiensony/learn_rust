#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub struct Record {
    pub label: String,
    pub revision: usize,
}

pub fn marks_removed_at_both_ends_hash(record: &Record) -> usize {
    0
}
