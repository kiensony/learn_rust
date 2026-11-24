// rust-track: handwritten task-2891
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2891;

pub fn signed_sensor_magnitude_trace(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .chunks_exact(2)
            .map(|c| u32::from(i16::from_le_bytes([c[0], c[1]]).unsigned_abs()))
            .collect()
    };
    samples
}
