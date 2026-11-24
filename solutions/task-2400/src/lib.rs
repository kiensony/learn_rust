// rust-track: handwritten task-2400
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2400;

pub fn last_change_age_replay_mismatch(input: &[u8], replay: &[u32]) -> Option<usize> {
    let observations: Vec<u32> = {
        let mut previous = None;
        let mut run = 0;
        input
            .iter()
            .map(|&x| {
                run = if previous == Some(x) { run + 1 } else { 1 };
                previous = Some(x);
                run
            })
            .collect()
    };
    let first_difference = observations.iter().zip(replay).position(|(a, b)| a != b);
    first_difference.or_else(|| {
        (observations.len() != replay.len()).then_some(observations.len().min(replay.len()))
    })
}
