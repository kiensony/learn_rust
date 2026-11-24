#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn hot_code_addresses_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let mut samples = std::collections::BTreeMap::new();
        for &addr in input {
            *samples.entry(addr / 2).or_insert(0u32) += 1;
        }
        samples
            .into_iter()
            .filter(|(_, c)| *c >= 2)
            .map(|(k, _)| u32::from(k))
            .collect()
    };
    samples.into_iter().map(u64::from).sum()
}
