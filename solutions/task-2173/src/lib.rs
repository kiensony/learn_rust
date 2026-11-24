// rust-track: handwritten task-2173
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2173;

pub fn prefix_comparison_budget_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = { (0..input.len()).map(|i| i as u32).collect() };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
