// rust-track: handwritten task-1531
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1531;

pub fn packet_gray_code_trace(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = { input.iter().map(|&x| u32::from(wire::decode(x))).collect() };
    samples
}
