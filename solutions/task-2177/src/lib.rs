// rust-track: handwritten task-2177
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2177;

pub fn prefix_comparison_budget_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = { (0..input.len()).map(|i| i as u32).collect() };
    samples.sort_unstable();
    samples.dedup();
    samples
}
