// rust-track: handwritten task-1350
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1350;

pub fn packed_booleans_replay_mismatch(input: &str, replay: &[u32]) -> Option<usize> {
    let observations: Vec<u32> = {
        input
            .split(',')
            .filter_map(|s| s.trim().parse::<u8>().ok())
            .map(|x| x.count_ones())
            .collect()
    };
    let first_difference = observations.iter().zip(replay).position(|(a, b)| a != b);
    first_difference.or_else(|| {
        (observations.len() != replay.len()).then_some(observations.len().min(replay.len()))
    })
}
