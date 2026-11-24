// rust-track: handwritten task-1518
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1518;

pub fn packet_low_nibble_steps(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = { input.iter().map(|&x| u32::from(wire::decode(x))).collect() };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
