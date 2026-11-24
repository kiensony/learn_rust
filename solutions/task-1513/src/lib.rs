// rust-track: handwritten task-1513
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1513;

pub fn packet_low_nibble_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = { input.iter().map(|&x| u32::from(wire::decode(x))).collect() };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
