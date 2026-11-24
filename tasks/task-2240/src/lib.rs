#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn zero_copy_lines_reverse(input: &str) -> Vec<u32> {
    let samples: Vec<u32> = { input.lines().map(|l| l.chars().count() as u32).collect() };
    samples.into_iter().rev().collect()
}
