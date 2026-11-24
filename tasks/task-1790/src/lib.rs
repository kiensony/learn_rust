#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn token_multiplier_reverse(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let units: u32 = arity::arity!(unit unit unit unit);
        input
            .iter()
            .map(|&x| u32::from(x.saturating_sub(1)) * units)
            .collect()
    };
    samples.into_iter().rev().collect()
}
