// rust-track: handwritten task-2301
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2301;

pub fn warmup_discard_trace(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = { input.iter().skip(2).copied().map(u32::from).collect() };
    samples
}
