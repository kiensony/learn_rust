// rust-track: handwritten task-2308
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2308;

pub fn warmup_discard_steps(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = { input.iter().skip(2).copied().map(u32::from).collect() };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
