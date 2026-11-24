#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn warmup_discard_steps(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = { input.iter().skip(2).copied().map(u32::from).collect() };
    let _ = samples;
    Vec::new()
}
