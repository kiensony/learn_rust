#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn signed_sensor_magnitude_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        input
            .windows(2)
            .map(|c| u32::from(i16::from_le_bytes([c[0], c[1]]).unsigned_abs()))
            .collect()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
