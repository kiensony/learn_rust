// rust-track: handwritten task-1622
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1622;

pub fn round_trip_property_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        input
            .iter()
            .map(|&x| u32::from(x.wrapping_mul(2) / 2 != x))
            .collect()
    };
    samples.into_iter().map(u64::from).sum()
}
