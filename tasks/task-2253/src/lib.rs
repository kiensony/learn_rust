#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn cpu_sample_counts_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        let mut samples = std::collections::BTreeMap::new();
        for &addr in input {
            *samples.entry(addr / 2).or_insert(0u32) += 1;
        }
        samples.into_iter().map(|(_, c)| c).collect()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
