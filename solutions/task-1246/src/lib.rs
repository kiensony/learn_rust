// rust-track: handwritten task-1246
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1246;

pub fn line_byte_lengths_threshold_crossing(input: &str, limit: u32) -> Option<(usize, u32)> {
    let observations: Vec<u32> = {
        use std::io::{BufRead, Cursor};
        Cursor::new(input)
            .lines()
            .map(|l| l.unwrap().len() as u32)
            .collect()
    };
    observations
        .into_iter()
        .enumerate()
        .find(|&(_, value)| value >= limit)
}
