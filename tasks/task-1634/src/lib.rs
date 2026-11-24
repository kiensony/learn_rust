#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn clamp_property_peaks(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            input
                .iter()
                .map(|&x| u32::from(x.clamp(4, 16) != x))
                .collect()
        };
        samples.sort_unstable();
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
