// rust-track: handwritten task-1610
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1610;

pub fn parity_property_reverse(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = { input.iter().map(|&x| u32::from(x % 2 != (x & 1))).collect() };
    samples.into_iter().rev().collect()
}
