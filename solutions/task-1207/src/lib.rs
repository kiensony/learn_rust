// rust-track: handwritten task-1207
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1207;

pub fn line_byte_lengths_distinct(input: &str) -> Vec<u32> {
    let mut samples = observe_line_byte_lengths(input);
    samples.sort_unstable();
    samples.dedup();
    samples
}

fn observe_line_byte_lengths(input: &str) -> Vec<u32> {
    use std::io::{BufRead, Cursor};
    Cursor::new(input)
        .lines()
        .map(|l| l.unwrap().len() as u32)
        .collect()
}
