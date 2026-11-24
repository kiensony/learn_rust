// rust-track: handwritten task-1563
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1563;

pub fn feature_padding_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        let stride = if cfg!(feature = "wide") { 4u32 } else { 1u32 };
        input.iter().map(|&b| u32::from(b) * stride).collect()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
