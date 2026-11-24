// rust-track: handwritten task-1223
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1223;

pub fn delimited_fields_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        use std::io::{BufRead, Cursor};
        Cursor::new(input)
            .split(0)
            .map(|s| s.unwrap().len() as u32)
            .collect()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
