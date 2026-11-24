// rust-track: handwritten task-1227
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1227;

pub fn delimited_fields_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        use std::io::{BufRead, Cursor};
        Cursor::new(input)
            .split(0)
            .map(|s| s.unwrap().len() as u32)
            .collect()
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
