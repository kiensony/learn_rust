// rust-track: handwritten task-2296
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2296;

pub fn cpu_sample_counts_threshold_crossing(input: &[u8], limit: u32) -> Option<(usize, u32)> {
    let observations: Vec<u32> = {
        let mut samples = std::collections::BTreeMap::new();
        for &addr in input {
            *samples.entry(addr).or_insert(0u32) += 1;
        }
        samples.into_iter().map(|(_, c)| c).collect()
    };
    observations
        .into_iter()
        .enumerate()
        .find(|&(_, value)| value >= limit)
}
