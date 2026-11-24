// rust-track: handwritten task-1781
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1781;

pub fn token_multiplier_trace(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let units: u32 = arity::arity!(unit unit unit unit);
        input.iter().map(|&x| u32::from(x) * units).collect()
    };
    samples
}
