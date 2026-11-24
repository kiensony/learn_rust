// rust-track: handwritten task-2263
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2263;

pub fn hot_code_addresses_bounds(input: &[u8]) -> Option<(u32, u32)> {
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
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
