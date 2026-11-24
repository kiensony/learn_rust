// rust-track: handwritten task-2119
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2119;

pub fn priority_removal_longest_run(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let mut heap: std::collections::BinaryHeap<_> = input.iter().copied().collect();
        std::iter::from_fn(|| heap.pop()).map(u32::from).collect()
    };
    let (mut previous, mut run, mut longest) = (None, 0u64, 0);
    for x in samples {
        run = if previous == Some(x) { run + 1 } else { 1 };
        longest = longest.max(run);
        previous = Some(x);
    }
    longest
}
