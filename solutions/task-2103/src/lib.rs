// rust-track: handwritten task-2103
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2103;

pub fn sorted_keys_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        let set: std::collections::BTreeSet<_> = input.iter().copied().collect();
        set.into_iter().map(u32::from).collect()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
