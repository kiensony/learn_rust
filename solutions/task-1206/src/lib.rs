// rust-track: handwritten task-1206
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1206;

pub fn line_byte_lengths_descending_steps(input: &str) -> u64 {
    let samples: Vec<u32> = {
        use std::io::{BufRead, Cursor};
        Cursor::new(input)
            .lines()
            .map(|l| l.unwrap().len() as u32)
            .collect()
    };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
