// rust-track: handwritten task-2266
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2266;

pub fn hot_code_addresses_descending_steps(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let mut samples = std::collections::BTreeMap::new();
        for &addr in input {
            *samples.entry(addr).or_insert(0u32) += 1;
        }
        samples
            .into_iter()
            .filter(|(_, c)| *c >= 2)
            .map(|(k, _)| u32::from(k))
            .collect()
    };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
