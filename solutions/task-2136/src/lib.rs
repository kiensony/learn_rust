// rust-track: handwritten task-2136
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2136;

pub fn histogram_bins_descending_steps(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let mut map = std::collections::BTreeMap::new();
        for &x in input {
            *map.entry(x).or_insert(0u32) += 1
        }
        map.into_values().collect()
    };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
