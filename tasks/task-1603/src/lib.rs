#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn parity_property_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        input
            .iter()
            .map(|&x| 1 - u32::from(x % 2 != (x & 1)))
            .collect()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
