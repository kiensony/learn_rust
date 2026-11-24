// rust-track: handwritten task-2174
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2174;

pub fn prefix_comparison_budget_peaks(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = { (0..input.len()).map(|i| i as u32).collect() };
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
