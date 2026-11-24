// rust-track: handwritten task-1241
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1241;

pub fn limited_reads_trace(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        use std::io::{Cursor, Read};
        let mut prefix = Vec::new();
        Cursor::new(input).take(4).read_to_end(&mut prefix).unwrap();
        prefix.into_iter().map(u32::from).collect()
    };
    samples
}
