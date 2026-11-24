#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn duplicate_insertion_cost_bounds(input: &[u8]) -> Option<(u32, u32)> {
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
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
