// rust-track: handwritten task-1506
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1506;

pub fn packet_high_nibble_descending_steps(input: &[u8]) -> u64 {
    let samples: Vec<u32> = { input.iter().map(|&x| u32::from(wire::decode(x))).collect() };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
