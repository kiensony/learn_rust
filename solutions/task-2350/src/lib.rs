// rust-track: handwritten task-2350
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2350;

pub fn baseline_drift_replay_mismatch(input: &[u8], replay: &[u32]) -> Option<usize> {
    let observations: Vec<u32> = {
        let base = input.first().copied().unwrap_or(0);
        input.iter().map(|&x| u32::from(x.abs_diff(base))).collect()
    };
    let first_difference = observations.iter().zip(replay).position(|(a, b)| a != b);
    first_difference.or_else(|| {
        (observations.len() != replay.len()).then_some(observations.len().min(replay.len()))
    })
}
