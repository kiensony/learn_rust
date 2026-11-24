// rust-track: handwritten task-1201
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1201;

pub fn line_byte_lengths_trace(input: &str) -> Vec<u32> {
    let samples: Vec<u32> = {
        use std::io::{BufRead, Cursor};
        Cursor::new(input)
            .lines()
            .map(|l| l.unwrap().len() as u32)
            .collect()
    };
    samples
}
