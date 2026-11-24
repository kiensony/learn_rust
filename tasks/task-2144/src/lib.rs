#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn first_occurrences_peaks(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            let mut seen = std::collections::HashSet::new();
            input
                .iter()
                .filter(|&&x| seen.insert(x))
                .map(|&x| u32::from(x))
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
