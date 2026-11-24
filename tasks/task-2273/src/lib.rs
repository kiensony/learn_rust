#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn exclusive_hotspots_bounds(input: &[u8]) -> Option<(u32, u32)> {
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
    let _ = samples;
    None
}
