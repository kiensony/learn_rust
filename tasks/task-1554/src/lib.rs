#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn feature_scaling_peaks(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let _ = input;
        Vec::<u32>::new()
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
