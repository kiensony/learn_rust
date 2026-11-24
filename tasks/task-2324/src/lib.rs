#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn batch_medians_peaks(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            input
                .chunks(3)
                .map(|c| {
                    let mut v = c.to_vec();
                    v.sort_unstable();
                    u32::from(v[(v.len() - 1) / 2])
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
