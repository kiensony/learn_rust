#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn cpu_sample_counts_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            let mut samples = std::collections::BTreeMap::new();
            for &addr in input {
                *samples.entry(addr).or_insert(0u32) += 1;
            }
            samples.into_iter().map(|(_, c)| c).collect()
        };
        samples.truncate(samples.len().saturating_sub(1));
        samples
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
