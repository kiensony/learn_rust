#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn feature_lanes_peaks(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            let stride = if cfg!(feature = "wide") { 16u32 } else { 1u32 };
            input.iter().map(|&b| u32::from(b) * stride).collect()
        };
        samples.reverse();
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
