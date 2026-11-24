// rust-track: handwritten task-2200
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2200;

pub fn duplicate_insertion_cost_replay_mismatch(input: &[u8], replay: &[u32]) -> Option<usize> {
    let observations: Vec<u32> = {
        let mut seen = std::collections::BTreeSet::new();
        input
            .iter()
            .map(|&x| {
                seen.insert(x);
                seen.len() as u32
            })
            .collect()
    };
    let first_difference = observations.iter().zip(replay).position(|(a, b)| a != b);
    first_difference.or_else(|| {
        (observations.len() != replay.len()).then_some(observations.len().min(replay.len()))
    })
}
