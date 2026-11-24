// rust-track: handwritten task-1226
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1226;

pub fn delimited_fields_descending_steps(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        use std::io::{BufRead, Cursor};
        Cursor::new(input)
            .split(0)
            .map(|s| s.unwrap().len() as u32)
            .collect()
    };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
