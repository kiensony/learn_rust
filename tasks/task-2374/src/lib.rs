#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn cumulative_occupancy_peaks(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            let mut n = 0;
            input
                .iter()
                .map(|&x| {
                    n += u32::from(x != 0);
                    n
                })
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
