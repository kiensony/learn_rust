#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub struct Record {
    pub label: String,
    pub revision: usize,
}

pub fn leading_marks_removed_pipe(record: &Record) -> usize {
    0
}
