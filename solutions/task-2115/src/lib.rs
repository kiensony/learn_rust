// rust-track: handwritten task-2115
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2115;

pub fn priority_removal_changes(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        let mut heap: std::collections::BinaryHeap<_> = input.iter().copied().collect();
        std::iter::from_fn(|| heap.pop()).map(u32::from).collect()
    };
    samples.dedup();
    samples
}
