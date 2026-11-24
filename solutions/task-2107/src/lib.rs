// rust-track: handwritten task-2107
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2107;

pub fn sorted_keys_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples = observe_sorted_keys(input);
    samples.sort_unstable();
    samples.dedup();
    samples
}

fn observe_sorted_keys(input: &[u8]) -> Vec<u32> {
    let set: std::collections::BTreeSet<_> = input.iter().copied().collect();
    set.into_iter().map(u32::from).collect()
}
