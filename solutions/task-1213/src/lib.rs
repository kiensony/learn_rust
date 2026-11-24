// rust-track: handwritten task-1213
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1213;

pub fn read_chunks_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        use std::io::{Cursor, Read};
        let mut reader = Cursor::new(input);
        let mut block = [0; 3];
        let mut lengths = Vec::new();
        loop {
            let n = reader.read(&mut block).unwrap();
            if n == 0 {
                break;
            }
            lengths.push(n as u32);
        }
        lengths
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
