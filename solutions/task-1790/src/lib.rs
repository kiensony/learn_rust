// rust-track: handwritten task-1790
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1790;

pub fn token_multiplier_reverse(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let units: u32 = arity::arity!(unit unit unit unit);
        input.iter().map(|&x| u32::from(x) * units).collect()
    };
    samples.into_iter().rev().collect()
}
