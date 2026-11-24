// rust-track: handwritten task-2175
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2175;

pub fn prefix_comparison_budget_changes(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = { (0..input.len()).map(|i| i as u32).collect() };
    samples.dedup();
    samples
}
