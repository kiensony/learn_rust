// rust-track: handwritten task-1450
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1450;

pub fn deadline_slack_replay_mismatch(input: &[u8], replay: &[u32]) -> Option<usize> {
    let observations: Vec<u32> = {
        use std::time::Duration;
        input
            .iter()
            .map(|&ticks| {
                Duration::from_millis(u64::from(ticks) * 17)
                    .abs_diff(Duration::from_millis(250))
                    .as_millis() as u32
            })
            .collect()
    };
    let first_difference = observations.iter().zip(replay).position(|(a, b)| a != b);
    first_difference.or_else(|| {
        (observations.len() != replay.len()).then_some(observations.len().min(replay.len()))
    })
}
