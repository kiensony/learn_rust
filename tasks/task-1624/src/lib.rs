#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn round_trip_property_peaks(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .iter()
            .map(|&x| 1 - u32::from(x.wrapping_mul(2) / 2 != x))
            .collect()
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
