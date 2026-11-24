// rust-track: handwritten task-2300
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2300;

pub fn profile_ranking_replay_mismatch(input: &[u8], replay: &[u32]) -> Option<usize> {
    let observations: Vec<u32> = {
        let mut samples = std::collections::BTreeMap::new();
        for &addr in input {
            *samples.entry(addr).or_insert(0u32) += 1;
        }
        samples
            .into_iter()
            .map(|(k, c)| (c, k))
            .collect::<std::collections::BinaryHeap<_>>()
            .into_sorted_vec()
            .into_iter()
            .rev()
            .map(|(_, k)| u32::from(k))
            .collect()
    };
    let first_difference = observations.iter().zip(replay).position(|(a, b)| a != b);
    first_difference.or_else(|| {
        (observations.len() != replay.len()).then_some(observations.len().min(replay.len()))
    })
}
