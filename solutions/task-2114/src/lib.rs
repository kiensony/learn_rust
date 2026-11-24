// rust-track: handwritten task-2114
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2114;

pub fn priority_removal_peaks(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut heap: std::collections::BinaryHeap<_> = input.iter().copied().collect();
        std::iter::from_fn(|| heap.pop()).map(u32::from).collect()
    };
    let mut peak = None;
    samples
        .into_iter()
        .filter(|&x| {
            if peak.is_none_or(|p| x > p) {
                peak = Some(x);
                true
            } else {
                false
            }
        })
        .collect()
}
