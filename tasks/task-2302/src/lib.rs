#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn warmup_discard_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = { input.iter().skip(2).copied().map(u32::from).collect() };
    let _ = samples;
    0
}
