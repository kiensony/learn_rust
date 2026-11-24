// rust-track: handwritten task-2232
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2232;

pub fn zero_copy_lines_total(input: &str) -> u64 {
    let samples: Vec<u32> = { input.lines().map(|l| l.len() as u32).collect() };
    samples.into_iter().map(u64::from).sum()
}
