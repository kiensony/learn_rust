// rust-track: handwritten task-1600
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1600;

pub fn feature_pages_replay_mismatch(input: &[u8], replay: &[u32]) -> Option<usize> {
    let observations: Vec<u32> = {
        let stride = if cfg!(feature = "wide") { 32u32 } else { 1u32 };
        input.iter().map(|&b| u32::from(b) * stride).collect()
    };
    let first_difference = observations.iter().zip(replay).position(|(a, b)| a != b);
    first_difference.or_else(|| {
        (observations.len() != replay.len()).then_some(observations.len().min(replay.len()))
    })
}
