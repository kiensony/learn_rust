// rust-track: handwritten task-1212
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1212;

pub fn read_chunks_total(input: &[u8]) -> u64 {
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
    samples.into_iter().map(u64::from).sum()
}
