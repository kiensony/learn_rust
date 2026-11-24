// rust-track: handwritten task-1800
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1800;

pub fn token_offset_replay_mismatch(input: &[u8], replay: &[u32]) -> Option<usize> {
    let observations: Vec<u32> = {
        let units: u32 = arity::arity!(unit unit unit unit unit);
        input.iter().map(|&x| u32::from(x) + units).collect()
    };
    let first_difference = observations.iter().zip(replay).position(|(a, b)| a != b);
    first_difference.or_else(|| {
        (observations.len() != replay.len()).then_some(observations.len().min(replay.len()))
    })
}
