// rust-track: handwritten task-2291
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2291;

pub fn profile_ranking_trace(input: &[u8]) -> Vec<u32> {
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
}
