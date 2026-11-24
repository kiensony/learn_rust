// rust-track: handwritten task-1650
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1650;

pub fn shift_property_replay_mismatch(input: &[u8], replay: &[u32]) -> Option<usize> {
    let observations: Vec<u32> = {
        input
            .iter()
            .map(|&x| u32::from((x >> 1) << 1 != x))
            .collect()
    };
    let first_difference = observations.iter().zip(replay).position(|(a, b)| a != b);
    first_difference.or_else(|| {
        (observations.len() != replay.len()).then_some(observations.len().min(replay.len()))
    })
}
