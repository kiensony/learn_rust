#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn exclusive_hotspots_peaks(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut samples = std::collections::BTreeMap::new();
        for &addr in input {
            *samples.entry(addr / 2).or_insert(0u32) += 1;
        }
        samples
            .into_iter()
            .filter(|(_, c)| *c == 1)
            .map(|(k, _)| u32::from(k))
            .collect()
    };
    let mut peak = None;
    samples
        .into_iter()
        .filter(|&x| {
            if peak.is_none_or(|p| x > p) {
                peak = Some(x);
                true
            } else {
                false
            }
        })
        .collect()
}
