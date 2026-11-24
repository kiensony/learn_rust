#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn zero_copy_lines_bounds(input: &str) -> Option<(u32, u32)> {
    let samples: Vec<u32> = { input.lines().map(|l| l.chars().count() as u32).collect() };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
