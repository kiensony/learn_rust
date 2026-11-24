#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn priority_removal_trace(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut heap: std::collections::BinaryHeap<_> = input.iter().copied().collect();
        std::iter::from_fn(|| heap.pop()).map(u32::from).collect()
    };
    let _ = samples;
    Vec::new()
}
