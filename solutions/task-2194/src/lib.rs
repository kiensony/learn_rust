// rust-track: handwritten task-2194
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2194;

pub fn duplicate_insertion_cost_peaks(input: &[u8]) -> Vec<u32> {
    let samples = observe_duplicate_insertion_cost(input);
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

fn observe_duplicate_insertion_cost(input: &[u8]) -> Vec<u32> {
    let mut seen = std::collections::BTreeSet::new();
    input
        .iter()
        .map(|&x| {
            seen.insert(x);
            seen.len() as u32
        })
        .collect()
}
