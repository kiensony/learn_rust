// rust-track: handwritten task-2310
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2310;

pub fn warmup_discard_reverse(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = { input.iter().skip(2).copied().map(u32::from).collect() };
    samples.into_iter().rev().collect()
}
