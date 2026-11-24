#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn embedded_flags_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = { input.iter().map(|&x| u32::from(x)).collect() };
    samples.into_iter().map(u64::from).sum()
}
