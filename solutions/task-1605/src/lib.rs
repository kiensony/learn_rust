// rust-track: handwritten task-1605
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1605;

pub fn parity_property_changes(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = { input.iter().map(|&x| u32::from(x % 2 != (x & 1))).collect() };
    samples.dedup();
    samples
}
