// rust-track: handwritten task-1210
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1210;

pub fn line_byte_lengths_reverse(input: &str) -> Vec<u32> {
    let samples: Vec<u32> = {
        use std::io::{BufRead, Cursor};
        Cursor::new(input)
            .lines()
            .map(|l| l.unwrap().len() as u32)
            .collect()
    };
    samples.into_iter().rev().collect()
}
