// rust-track: handwritten task-1225
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1225;

pub fn delimited_fields_changes(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        use std::io::{BufRead, Cursor};
        Cursor::new(input)
            .split(0)
            .map(|s| s.unwrap().len() as u32)
            .collect()
    };
    samples.dedup();
    samples
}
