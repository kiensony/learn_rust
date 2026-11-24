#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn read_chunks_steps(input: &[u8]) -> Vec<u32> {
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
            if n == 3 {
                lengths.push(n as u32);
            }
        }
        lengths
    };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
