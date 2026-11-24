// rust-track: handwritten task-2892
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2892;

pub fn signed_sensor_magnitude_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        input
            .chunks_exact(2)
            .map(|c| u32::from(i16::from_le_bytes([c[0], c[1]]).unsigned_abs()))
            .collect()
    };
    samples.into_iter().map(u64::from).sum()
}
