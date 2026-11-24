// rust-track: handwritten task-1230
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1230;

pub fn delimited_fields_reverse(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        use std::io::{BufRead, Cursor};
        Cursor::new(input)
            .split(0)
            .map(|s| s.unwrap().len() as u32)
            .collect()
    };
    samples.into_iter().rev().collect()
}
