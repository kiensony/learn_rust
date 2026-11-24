// rust-track: handwritten task-2193
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2193;

pub fn duplicate_insertion_cost_bounds(input: &[u8]) -> Option<(u32, u32)> {
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
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
