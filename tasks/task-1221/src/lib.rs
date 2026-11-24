#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn delimited_fields_trace(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        use std::io::{BufRead, Cursor};
        Cursor::new(input)
            .split(255)
            .map(|s| s.unwrap().len() as u32)
            .collect()
    };
    samples
}
