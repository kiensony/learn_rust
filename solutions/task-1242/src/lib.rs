// rust-track: handwritten task-1242
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1242;

pub fn limited_reads_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        use std::io::{Cursor, Read};
        let mut prefix = Vec::new();
        Cursor::new(input).take(4).read_to_end(&mut prefix).unwrap();
        prefix.into_iter().map(u32::from).collect()
    };
    samples.into_iter().map(u64::from).sum()
}
