// rust-track: handwritten task-1550
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1550;

pub fn packet_reversal_replay_mismatch(input: &[u8], replay: &[u32]) -> Option<usize> {
    let observations: Vec<u32> = { input.iter().map(|&x| u32::from(wire::decode(x))).collect() };
    let first_difference = observations.iter().zip(replay).position(|(a, b)| a != b);
    first_difference.or_else(|| {
        (observations.len() != replay.len()).then_some(observations.len().min(replay.len()))
    })
}
