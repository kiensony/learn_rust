// rust-track: handwritten task-1233
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1233;

pub fn seek_offsets_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        use std::io::{Cursor, Read, Seek, SeekFrom};
        let mut file = Cursor::new(input);
        let mut data = vec![];
        for i in (1..input.len()).step_by(2) {
            file.seek(SeekFrom::Start(i as u64)).unwrap();
            let mut b = [0u8];
            file.read_exact(&mut b).unwrap();
            data.push(u32::from(b[0]));
        }
        data
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
