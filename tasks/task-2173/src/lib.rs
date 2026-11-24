#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn prefix_comparison_budget_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = { (0..input.len()).map(|i| i as u32).collect() };
    let _ = samples;
    None
}
