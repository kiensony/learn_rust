// rust-track: handwritten task-2283
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2283;

pub fn weighted_pressure_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        let mut samples = std::collections::BTreeMap::new();
        for &addr in input {
            *samples.entry(addr).or_insert(0u32) += 1;
        }
        samples.into_iter().map(|(k, c)| u32::from(k) * c).collect()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
