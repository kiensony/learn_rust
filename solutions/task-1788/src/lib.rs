// rust-track: handwritten task-1788
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1788;

pub fn token_multiplier_steps(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let units: u32 = arity::arity!(unit unit unit unit);
        input.iter().map(|&x| u32::from(x) * units).collect()
    };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
