// rust-track: handwritten task-1512
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1512;

pub fn packet_low_nibble_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = { input.iter().map(|&x| u32::from(wire::decode(x))).collect() };
    samples.into_iter().map(u64::from).sum()
}
