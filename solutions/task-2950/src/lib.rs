// rust-track: handwritten task-2950
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2950;

pub fn rollback_triggers_replay_mismatch(input: &[u8], replay: &[u32]) -> Option<usize> {
    let observations: Vec<u32> = {
        input
            .chunks_exact(2)
            .map(|p| u32::from(u8::from(u16::from(p[1]) > 2 * u16::from(p[0]))))
            .collect()
    };
    let first_difference = observations.iter().zip(replay).position(|(a, b)| a != b);
    first_difference.or_else(|| {
        (observations.len() != replay.len()).then_some(observations.len().min(replay.len()))
    })
}
