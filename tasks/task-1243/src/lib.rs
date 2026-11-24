#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn limited_reads_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        use std::io::{Cursor, Read};
        let mut prefix = Vec::new();
        Cursor::new(input).take(3).read_to_end(&mut prefix).unwrap();
        prefix.into_iter().map(u32::from).collect()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
