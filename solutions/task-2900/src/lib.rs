// rust-track: handwritten task-2900
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2900;

pub fn signed_sensor_magnitude_replay_mismatch(input: &[u8], replay: &[u32]) -> Option<usize> {
    let observations: Vec<u32> = {
        input
            .chunks_exact(2)
            .map(|c| u32::from(i16::from_le_bytes([c[0], c[1]]).unsigned_abs()))
            .collect()
    };
    let first_difference = observations.iter().zip(replay).position(|(a, b)| a != b);
    first_difference.or_else(|| {
        (observations.len() != replay.len()).then_some(observations.len().min(replay.len()))
    })
}
