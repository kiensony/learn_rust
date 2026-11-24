#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn histogram_bins_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let mut map = std::collections::BTreeMap::new();
        for &x in input {
            *map.entry(x).or_insert(0u32) += 1
        }
        map.into_values().collect()
    };
    let _ = samples;
    0
}
