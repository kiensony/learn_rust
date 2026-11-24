// rust-track: handwritten task-2146
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2146;

pub fn sorted_keys_threshold_crossing(input: &[u8], limit: u32) -> Option<(usize, u32)> {
    let observations: Vec<u32> = {
        let set: std::collections::BTreeSet<_> = input.iter().copied().collect();
        set.into_iter().map(u32::from).collect()
    };
    observations
        .into_iter()
        .enumerate()
        .find(|&(_, value)| value >= limit)
}
