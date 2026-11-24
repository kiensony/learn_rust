// rust-track: handwritten task-2236
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2236;

pub fn zero_copy_lines_descending_steps(input: &str) -> u64 {
    let samples: Vec<u32> = { input.lines().map(|l| l.len() as u32).collect() };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
