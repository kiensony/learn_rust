// rust-track: handwritten task-1520
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1520;

pub fn packet_low_nibble_reverse(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = { input.iter().map(|&x| u32::from(wire::decode(x))).collect() };
    samples.into_iter().rev().collect()
}
