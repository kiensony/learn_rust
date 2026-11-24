// rust-track: handwritten task-1613
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1613;

pub fn saturation_property_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        input
            .iter()
            .map(|&x| u32::from(x.saturating_add(1) == x))
            .collect()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
