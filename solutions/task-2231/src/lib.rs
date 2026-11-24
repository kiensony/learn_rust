// rust-track: handwritten task-2231
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2231;

pub fn zero_copy_lines_trace(input: &str) -> Vec<u32> {
    let samples: Vec<u32> = { input.lines().map(|l| l.len() as u32).collect() };
    samples
}
