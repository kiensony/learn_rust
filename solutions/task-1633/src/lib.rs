// rust-track: handwritten task-1633
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1633;

pub fn clamp_property_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        input
            .iter()
            .map(|&x| u32::from(x.clamp(4, 16) != x))
            .collect()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
