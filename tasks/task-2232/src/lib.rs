#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn zero_copy_lines_total(input: &str) -> u64 {
    let samples: Vec<u32> = { input.lines().map(|l| l.len() as u32).collect() };
    let _ = samples;
    0
}
