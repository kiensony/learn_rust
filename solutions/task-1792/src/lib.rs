// rust-track: handwritten task-1792
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1792;

pub fn token_offset_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let units: u32 = arity::arity!(unit unit unit unit unit);
        input.iter().map(|&x| u32::from(x) + units).collect()
    };
    samples.into_iter().map(u64::from).sum()
}
