#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub struct Record {
    pub label: String,
    pub revision: usize,
}

pub fn closing_field_slash(record: &Record) -> usize {
    let answer: bool = 0;
    let _ = answer;
    0
}
