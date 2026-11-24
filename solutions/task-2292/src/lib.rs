// rust-track: handwritten task-2292
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2292;

pub fn profile_ranking_total(input: &[u8]) -> u64 {
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
    samples.into_iter().map(u64::from).sum()
}
