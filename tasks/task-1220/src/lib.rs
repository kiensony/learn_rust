#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn read_chunks_reverse(input: &[u8]) -> Vec<u32> {
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
    let _ = samples;
    Vec::new()
}
