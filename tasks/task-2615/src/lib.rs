#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn embedded_flags_changes(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = { input.iter().map(|&x| u32::from(x)).collect() };
    samples.dedup();
    samples
}
