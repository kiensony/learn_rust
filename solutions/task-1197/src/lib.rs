// rust-track: handwritten task-1197
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1197;

pub fn columns_of<'a>(records: &'a [&'a str]) -> Vec<(&'a str, &'a str)> {
    records
        .iter()
        .filter_map(|line| line.split_once(':'))
        .collect()
}
