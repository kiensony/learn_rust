// rust-track: handwritten task-2138
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2138;

pub fn histogram_bins_steps(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut map = std::collections::BTreeMap::new();
        for &x in input {
            *map.entry(x).or_insert(0u32) += 1
        }
        map.into_values().collect()
    };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
