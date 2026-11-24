// rust-track: handwritten task-1630
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1630;

pub fn round_trip_property_reverse(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .iter()
            .map(|&x| u32::from(x.wrapping_mul(2) / 2 != x))
            .collect()
    };
    samples.into_iter().rev().collect()
}
