#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn failure_threshold_peaks(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            input
                .chunks_exact(2)
                .map(|p| u32::from(u8::from(p[0] < 100 && p[1] >= 100)))
                .collect()
        };
        samples.truncate(samples.len().saturating_sub(1));
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
