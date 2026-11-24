// rust-track: handwritten task-2192
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2192;

pub fn duplicate_insertion_cost_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let mut seen = std::collections::BTreeSet::new();
        input
            .iter()
            .map(|&x| {
                seen.insert(x);
                seen.len() as u32
            })
            .collect()
    };
    samples.into_iter().map(u64::from).sum()
}
