// rust-track: handwritten task-2306
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2306;

pub fn warmup_discard_descending_steps(input: &[u8]) -> u64 {
    let samples: Vec<u32> = { input.iter().skip(2).copied().map(u32::from).collect() };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
