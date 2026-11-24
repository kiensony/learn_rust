// rust-track: handwritten task-1208
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1208;

pub fn line_byte_lengths_steps(input: &str) -> Vec<u32> {
    let samples: Vec<u32> = {
        use std::io::{BufRead, Cursor};
        Cursor::new(input)
            .lines()
            .map(|l| l.unwrap().len() as u32)
            .collect()
    };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
