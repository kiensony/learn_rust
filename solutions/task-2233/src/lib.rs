// rust-track: handwritten task-2233
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2233;

pub fn zero_copy_lines_bounds(input: &str) -> Option<(u32, u32)> {
    let samples: Vec<u32> = { input.lines().map(|l| l.len() as u32).collect() };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
