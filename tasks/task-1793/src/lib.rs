#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn token_offset_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        let units: u32 = arity::arity!(unit unit unit unit unit);
        input
            .iter()
            .map(|&x| u32::from(x.saturating_sub(1)) + units)
            .collect()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
