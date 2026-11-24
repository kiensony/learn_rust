// rust-track: handwritten task-1541
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1541;

pub fn packet_reversal_trace(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = { input.iter().map(|&x| u32::from(wire::decode(x))).collect() };
    samples
}
