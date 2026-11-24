// rust-track: handwritten task-2137
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2137;

pub fn histogram_bins_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        let mut map = std::collections::BTreeMap::new();
        for &x in input {
            *map.entry(x).or_insert(0u32) += 1
        }
        map.into_values().collect()
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
