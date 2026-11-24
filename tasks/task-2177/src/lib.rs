#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn prefix_comparison_budget_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        let mut samples: Vec<u32> = { (0..input.len()).map(|i| i as u32).collect() };
        samples.truncate(samples.len().saturating_sub(1));
        samples
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
