// rust-track: handwritten task-1525
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1525;

pub fn packet_parity_changes(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = { input.iter().map(|&x| u32::from(wire::decode(x))).collect() };
    samples.dedup();
    samples
}
