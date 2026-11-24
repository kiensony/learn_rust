#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn line_byte_lengths_distinct(input: &str) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            use std::io::{BufRead, Cursor};
            Cursor::new(input)
                .lines()
                .map(|l| l.unwrap().len() as u32)
                .collect()
        };
        samples.truncate(samples.len().saturating_sub(1));
        samples
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
