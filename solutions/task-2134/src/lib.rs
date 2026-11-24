// rust-track: handwritten task-2134
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2134;

pub fn histogram_bins_peaks(input: &[u8]) -> Vec<u32> {
    let samples = observe_histogram_bins(input);
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

fn observe_histogram_bins(input: &[u8]) -> Vec<u32> {
    let mut map = std::collections::BTreeMap::new();
    for &x in input {
        *map.entry(x).or_insert(0u32) += 1
    }
    map.into_values().collect()
}
