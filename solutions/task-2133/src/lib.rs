// rust-track: handwritten task-2133
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2133;

pub fn histogram_bins_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        let mut map = std::collections::BTreeMap::new();
        for &x in input {
            *map.entry(x).or_insert(0u32) += 1
        }
        map.into_values().collect()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
