// rust-track: handwritten task-2237
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2237;

pub fn zero_copy_lines_distinct(input: &str) -> Vec<u32> {
    let mut samples: Vec<u32> = { input.lines().map(|l| l.len() as u32).collect() };
    samples.sort_unstable();
    samples.dedup();
    samples
}
