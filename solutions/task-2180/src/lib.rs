// rust-track: handwritten task-2180
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2180;

pub fn prefix_comparison_budget_reverse(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = { (0..input.len()).map(|i| i as u32).collect() };
    samples.into_iter().rev().collect()
}
