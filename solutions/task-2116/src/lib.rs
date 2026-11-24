// rust-track: handwritten task-2116
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2116;

pub fn priority_removal_descending_steps(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let mut heap: std::collections::BinaryHeap<_> = input.iter().copied().collect();
        std::iter::from_fn(|| heap.pop()).map(u32::from).collect()
    };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
