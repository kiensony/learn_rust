// rust-track: handwritten task-2267
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2267;

pub fn hot_code_addresses_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples = observe_hot_code_addresses(input);
    samples.sort_unstable();
    samples.dedup();
    samples
}

fn observe_hot_code_addresses(input: &[u8]) -> Vec<u32> {
    let mut samples = std::collections::BTreeMap::new();
    for &addr in input {
        *samples.entry(addr).or_insert(0u32) += 1;
    }
    samples
        .into_iter()
        .filter(|(_, c)| *c >= 2)
        .map(|(k, _)| u32::from(k))
        .collect()
}
