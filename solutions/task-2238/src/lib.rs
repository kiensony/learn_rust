// rust-track: handwritten task-2238
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2238;

pub fn zero_copy_lines_steps(input: &str) -> Vec<u32> {
    let samples: Vec<u32> = { input.lines().map(|l| l.len() as u32).collect() };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
