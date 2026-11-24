#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn weak_liveness_peaks(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            use std::sync::Arc;
            input
                .iter()
                .map(|&x| {
                    let data = Arc::new(x);
                    let weak = Arc::downgrade(&data);
                    let _other = (x % 2 == 1).then(|| Arc::clone(&data));
                    drop(data);
                    u32::from(weak.upgrade().is_some())
                })
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
