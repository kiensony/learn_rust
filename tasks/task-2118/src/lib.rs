#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn priority_removal_steps(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            let mut heap: std::collections::BinaryHeap<_> = input.iter().copied().collect();
            std::iter::from_fn(|| heap.pop()).map(u32::from).collect()
        };
        samples.reverse();
        samples
    };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
