#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub struct Record {
    pub label: String,
    pub revision: usize,
}

pub fn shortest_field_with_first_tie_colon(record: &Record) -> usize {
    0
}
