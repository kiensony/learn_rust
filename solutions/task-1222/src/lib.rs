// rust-track: handwritten task-1222
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1222;

pub fn delimited_fields_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        use std::io::{BufRead, Cursor};
        Cursor::new(input)
            .split(0)
            .map(|s| s.unwrap().len() as u32)
            .collect()
    };
    samples.into_iter().map(u64::from).sum()
}
