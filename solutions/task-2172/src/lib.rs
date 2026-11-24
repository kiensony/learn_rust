// rust-track: handwritten task-2172
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2172;

pub fn prefix_comparison_budget_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = { (0..input.len()).map(|i| i as u32).collect() };
    samples.into_iter().map(u64::from).sum()
}
