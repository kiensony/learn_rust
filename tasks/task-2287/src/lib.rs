#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn weighted_pressure_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        let mut samples = std::collections::BTreeMap::new();
        for &addr in input {
            *samples.entry(addr / 2).or_insert(0u32) += 1;
        }
        samples.into_iter().map(|(k, c)| u32::from(k) * c).collect()
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
