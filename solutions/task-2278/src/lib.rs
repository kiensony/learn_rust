// rust-track: handwritten task-2278
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2278;

pub fn exclusive_hotspots_steps(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut samples = std::collections::BTreeMap::new();
        for &addr in input {
            *samples.entry(addr).or_insert(0u32) += 1;
        }
        samples
            .into_iter()
            .filter(|(_, c)| *c == 1)
            .map(|(k, _)| u32::from(k))
            .collect()
    };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
