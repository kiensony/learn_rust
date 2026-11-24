// rust-track: handwritten task-1515
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1515;

pub fn packet_low_nibble_changes(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = { input.iter().map(|&x| u32::from(wire::decode(x))).collect() };
    samples.dedup();
    samples
}
