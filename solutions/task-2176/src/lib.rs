// rust-track: handwritten task-2176
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2176;

pub fn prefix_comparison_budget_descending_steps(input: &[u8]) -> u64 {
    let samples: Vec<u32> = { (0..input.len()).map(|i| i as u32).collect() };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
