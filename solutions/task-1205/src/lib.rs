// rust-track: handwritten task-1205
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1205;

pub fn line_byte_lengths_changes(input: &str) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        use std::io::{BufRead, Cursor};
        Cursor::new(input)
            .lines()
            .map(|l| l.unwrap().len() as u32)
            .collect()
    };
    samples.dedup();
    samples
}
