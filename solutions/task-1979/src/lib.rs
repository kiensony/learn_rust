// rust-track: handwritten task-1979
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1979;

pub fn atomic_toggle_longest_run(input: &[u8]) -> u64 {
    let samples = observe_atomic_toggle(input);
    let (mut previous, mut run, mut longest) = (None, 0u64, 0);
    for x in samples {
        run = if previous == Some(x) { run + 1 } else { 1 };
        longest = longest.max(run);
        previous = Some(x);
    }
    longest
}

fn observe_atomic_toggle(input: &[u8]) -> Vec<u32> {
    use std::sync::atomic::{AtomicU32, Ordering};
    let cell = AtomicU32::new(0);
    input
        .iter()
        .map(|&x| cell.fetch_xor(u32::from(x), Ordering::Relaxed))
        .collect()
}
