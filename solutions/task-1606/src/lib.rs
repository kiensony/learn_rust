// rust-track: handwritten task-1606
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1606;

pub fn parity_property_descending_steps(input: &[u8]) -> u64 {
    let samples: Vec<u32> = { input.iter().map(|&x| u32::from(x % 2 != (x & 1))).collect() };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
