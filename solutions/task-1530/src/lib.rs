// rust-track: handwritten task-1530
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1530;

pub fn packet_parity_reverse(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = { input.iter().map(|&x| u32::from(wire::decode(x))).collect() };
    samples.into_iter().rev().collect()
}
