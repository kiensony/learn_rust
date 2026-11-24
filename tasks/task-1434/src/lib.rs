#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn timer_windows_peaks(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            use std::time::Duration;
            input
                .iter()
                .map(|&ticks| {
                    Duration::from_millis(u64::from(ticks) * 17)
                        .as_millis()
                        .div_ceil(100) as u32
                })
                .collect()
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
