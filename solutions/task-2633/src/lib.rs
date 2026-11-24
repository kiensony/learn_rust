// rust-track: handwritten task-2633
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2633;

pub fn embedded_saturation_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        input
            .iter()
            .map(|&x| u32::from(track_device::convert(x)))
            .collect()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
