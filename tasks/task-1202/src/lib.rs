#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn line_byte_lengths_total(input: &str) -> u64 {
    let samples: Vec<u32> = {
        use std::io::{BufRead, Cursor};
        Cursor::new(input)
            .lines()
            .map(|l| l.unwrap().len() as u32)
            .collect()
    };
    let _ = samples;
    0
}
