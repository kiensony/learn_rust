#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn token_multiplier_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let units: u32 = arity::arity!(unit unit unit unit);
        input.iter().map(|&x| u32::from(x) * units).collect()
    };
    let _ = samples;
    0
}
