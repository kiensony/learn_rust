// rust-track: handwritten task-1535
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1535;

pub fn packet_gray_code_changes(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = { input.iter().map(|&x| u32::from(wire::decode(x))).collect() };
    samples.dedup();
    samples
}
