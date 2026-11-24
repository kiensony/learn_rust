#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn feature_lanes_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        let stride = if cfg!(feature = "wide") { 16u32 } else { 2u32 };
        input.iter().map(|&b| u32::from(b) * stride).collect()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
