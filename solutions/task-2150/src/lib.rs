// rust-track: handwritten task-2150
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2150;

pub fn first_occurrences_replay_mismatch(input: &[u8], replay: &[u32]) -> Option<usize> {
    let observations: Vec<u32> = {
        let mut seen = std::collections::HashSet::new();
        input
            .iter()
            .filter(|&&x| seen.insert(x))
            .map(|&x| u32::from(x))
            .collect()
    };
    let first_difference = observations.iter().zip(replay).position(|(a, b)| a != b);
    first_difference.or_else(|| {
        (observations.len() != replay.len()).then_some(observations.len().min(replay.len()))
    })
}
