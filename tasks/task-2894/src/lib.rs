#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn signed_sensor_magnitude_peaks(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            input
                .chunks_exact(2)
                .map(|c| u32::from(i16::from_le_bytes([c[0], c[1]]).unsigned_abs()))
                .collect()
        };
        if let Some(first) = samples.first().copied() {
            samples = vec![first];
        }
        samples
    };
    let mut peak = None;
    samples
        .into_iter()
        .filter(|&x| {
            if peak.is_none_or(|p| x > p) {
                peak = Some(x);
                true
            } else {
                false
            }
        })
        .collect()
}
