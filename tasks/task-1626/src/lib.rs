#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn round_trip_property_descending_steps(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        input
            .iter()
            .map(|&x| u32::from(x.wrapping_mul(2) / 2 != x))
            .collect()
    };
    let _ = samples;
    0
}
