// rust-track: handwritten task-1785
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1785;

pub fn token_multiplier_changes(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        let units: u32 = arity::arity!(unit unit unit unit);
        input.iter().map(|&x| u32::from(x) * units).collect()
    };
    samples.dedup();
    samples
}
