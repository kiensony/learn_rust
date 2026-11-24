// rust-track: handwritten task-2252
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2252;

pub fn cpu_sample_counts_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let mut samples = std::collections::BTreeMap::new();
        for &addr in input {
            *samples.entry(addr).or_insert(0u32) += 1;
        }
        samples.into_iter().map(|(_, c)| c).collect()
    };
    samples.into_iter().map(u64::from).sum()
}
