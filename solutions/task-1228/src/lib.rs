// rust-track: handwritten task-1228
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1228;

pub fn delimited_fields_steps(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        use std::io::{BufRead, Cursor};
        Cursor::new(input)
            .split(0)
            .map(|s| s.unwrap().len() as u32)
            .collect()
    };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
