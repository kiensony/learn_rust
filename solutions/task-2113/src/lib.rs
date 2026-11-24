// rust-track: handwritten task-2113
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2113;

pub fn priority_removal_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        let mut heap: std::collections::BinaryHeap<_> = input.iter().copied().collect();
        std::iter::from_fn(|| heap.pop()).map(u32::from).collect()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
