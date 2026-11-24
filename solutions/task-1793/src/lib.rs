// rust-track: handwritten task-1793
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1793;

pub fn token_offset_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        let units: u32 = arity::arity!(unit unit unit unit unit);
        input.iter().map(|&x| u32::from(x) + units).collect()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
