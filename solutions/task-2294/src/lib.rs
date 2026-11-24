// rust-track: handwritten task-2294
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2294;

pub fn profile_ranking_peaks(input: &[u8]) -> Vec<u32> {
    let samples = observe_profile_ranking(input);
    let mut peak = None;
    samples
        .into_iter()
        .filter(|&x| {
            if peak.is_none_or(|p| x > p) {
                peak = Some(x);
                true
            } else {
                false
            }
        })
        .collect()
}

fn observe_profile_ranking(input: &[u8]) -> Vec<u32> {
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
}
