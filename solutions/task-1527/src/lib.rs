// rust-track: handwritten task-1527
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1527;

pub fn packet_parity_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = { input.iter().map(|&x| u32::from(wire::decode(x))).collect() };
    samples.sort_unstable();
    samples.dedup();
    samples
}
