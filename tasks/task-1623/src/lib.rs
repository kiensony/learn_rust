#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn round_trip_property_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        input
            .iter()
            .map(|&x| u32::from(x.wrapping_mul(2) / 2 != x))
            .collect()
    };
    let _ = samples;
    None
}
