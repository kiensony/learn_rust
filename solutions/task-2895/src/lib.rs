// rust-track: handwritten task-2895
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2895;

pub fn signed_sensor_magnitude_changes(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        input
            .chunks_exact(2)
            .map(|c| u32::from(i16::from_le_bytes([c[0], c[1]]).unsigned_abs()))
            .collect()
    };
    samples.dedup();
    samples
}
