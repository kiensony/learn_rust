// rust-track: handwritten task-2101
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2101;

pub fn sorted_keys_trace(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let set: std::collections::BTreeSet<_> = input.iter().copied().collect();
        set.into_iter().map(u32::from).collect()
    };
    samples
}
