#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn packet_high_nibble_steps(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = { input.iter().map(|&x| u32::from(wire::decode(x))).collect() };
    let _ = samples;
    Vec::new()
}
