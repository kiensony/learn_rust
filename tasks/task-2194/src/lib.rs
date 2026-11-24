#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn duplicate_insertion_cost_peaks(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            let mut seen = std::collections::BTreeSet::new();
            input
                .iter()
                .map(|&x| {
                    seen.insert(x);
                    seen.len() as u32
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
