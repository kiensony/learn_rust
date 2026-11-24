// rust-track: handwritten task-2120
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2120;

pub fn priority_removal_reverse(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut heap: std::collections::BinaryHeap<_> = input.iter().copied().collect();
        std::iter::from_fn(|| heap.pop()).map(u32::from).collect()
    };
    samples.into_iter().rev().collect()
}
