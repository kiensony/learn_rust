// rust-track: handwritten task-2302
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2302;

pub fn warmup_discard_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = { input.iter().skip(2).copied().map(u32::from).collect() };
    samples.into_iter().map(u64::from).sum()
}
