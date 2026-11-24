#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn field_indexes_peaks(input: &str) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            input
                .split(',')
                .filter_map(|s| s.trim().parse::<u8>().ok())
                .enumerate()
                .filter(|(_, x)| *x == 0)
                .map(|(i, _)| i as u32)
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
