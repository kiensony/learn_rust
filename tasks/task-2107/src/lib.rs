#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn sorted_keys_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            let set: std::collections::BTreeSet<_> = input.iter().copied().collect();
            set.into_iter().map(u32::from).collect()
        };
        samples.truncate(samples.len().saturating_sub(1));
        samples
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
