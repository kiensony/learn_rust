// rust-track: handwritten task-2117
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2117;

pub fn priority_removal_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples = observe_priority_removal(input);
    samples.sort_unstable();
    samples.dedup();
    samples
}

fn observe_priority_removal(input: &[u8]) -> Vec<u32> {
    let mut heap: std::collections::BinaryHeap<_> = input.iter().copied().collect();
    std::iter::from_fn(|| heap.pop()).map(u32::from).collect()
}
