// rust-track: handwritten task-2178
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2178;

pub fn prefix_comparison_budget_steps(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = { (0..input.len()).map(|i| i as u32).collect() };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
