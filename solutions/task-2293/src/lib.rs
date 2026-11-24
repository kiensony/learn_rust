// rust-track: handwritten task-2293
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2293;

pub fn profile_ranking_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
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
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
