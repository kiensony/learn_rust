// rust-track: handwritten task-2147
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2147;

pub fn priority_removal_run_ledger(input: &[u8]) -> Vec<(u32, usize)> {
    let observations: Vec<u32> = {
        let mut heap: std::collections::BinaryHeap<_> = input.iter().copied().collect();
        std::iter::from_fn(|| heap.pop()).map(u32::from).collect()
    };
    let mut runs: Vec<(u32, usize)> = Vec::new();
    for value in observations {
        if let Some(last) = runs.last_mut().filter(|last| last.0 == value) {
            last.1 += 1;
        } else {
            runs.push((value, 1));
        }
    }
    runs
}
