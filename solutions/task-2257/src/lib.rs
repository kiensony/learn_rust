// rust-track: handwritten task-2257
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2257;

pub fn cpu_sample_counts_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples = observe_cpu_sample_counts(input);
    samples.sort_unstable();
    samples.dedup();
    samples
}

fn observe_cpu_sample_counts(input: &[u8]) -> Vec<u32> {
    let mut samples = std::collections::BTreeMap::new();
    for &addr in input {
        *samples.entry(addr).or_insert(0u32) += 1;
    }
    samples.into_iter().map(|(_, c)| c).collect()
}
