#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub struct Record {
    pub label: String,
    pub revision: usize,
}

pub fn first_nonempty_field_pipe(record: &Record) -> usize {
    0
}
