// rust-track: handwritten task-1542
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1542;

pub fn packet_reversal_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = { input.iter().map(|&x| u32::from(wire::decode(x))).collect() };
    samples.into_iter().map(u64::from).sum()
}
