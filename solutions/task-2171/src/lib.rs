// rust-track: handwritten task-2171
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2171;

pub fn prefix_comparison_budget_trace(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = { (0..input.len()).map(|i| i as u32).collect() };
    samples
}
