#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn last_change_age_peaks(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            let mut previous = None;
            let mut run = 0;
            input
                .iter()
                .map(|&x| {
                    run = if previous == Some(x) { run + 1 } else { 1 };
                    previous = Some(x);
                    run
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
