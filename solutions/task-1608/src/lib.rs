// rust-track: handwritten task-1608
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1608;

pub fn parity_property_steps(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = { input.iter().map(|&x| u32::from(x % 2 != (x & 1))).collect() };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
