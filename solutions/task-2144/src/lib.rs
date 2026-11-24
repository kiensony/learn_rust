// rust-track: handwritten task-2144
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2144;

pub fn first_occurrences_peaks(input: &[u8]) -> Vec<u32> {
    let samples = observe_first_occurrences(input);
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

fn observe_first_occurrences(input: &[u8]) -> Vec<u32> {
    let mut seen = std::collections::HashSet::new();
    input
        .iter()
        .filter(|&&x| seen.insert(x))
        .map(|&x| u32::from(x))
        .collect()
}
