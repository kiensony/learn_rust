// rust-track: handwritten task-2102
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2102;

pub fn sorted_keys_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let set: std::collections::BTreeSet<_> = input.iter().copied().collect();
        set.into_iter().map(u32::from).collect()
    };
    samples.into_iter().map(u64::from).sum()
}
