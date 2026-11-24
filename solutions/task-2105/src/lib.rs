// rust-track: handwritten task-2105
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2105;

pub fn sorted_keys_changes(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        let set: std::collections::BTreeSet<_> = input.iter().copied().collect();
        set.into_iter().map(u32::from).collect()
    };
    samples.dedup();
    samples
}
