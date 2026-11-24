// rust-track: handwritten task-1250
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1250;

pub fn limited_reads_replay_mismatch(input: &[u8], replay: &[u32]) -> Option<usize> {
    let observations: Vec<u32> = {
        use std::io::{Cursor, Read};
        let mut prefix = Vec::new();
        Cursor::new(input).take(4).read_to_end(&mut prefix).unwrap();
        prefix.into_iter().map(u32::from).collect()
    };
    let first_difference = observations.iter().zip(replay).position(|(a, b)| a != b);
    first_difference.or_else(|| {
        (observations.len() != replay.len()).then_some(observations.len().min(replay.len()))
    })
}
