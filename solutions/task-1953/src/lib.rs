// rust-track: handwritten task-1953
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1953;

pub fn atomic_running_total_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        use std::sync::atomic::{AtomicU32, Ordering};
        let cell = AtomicU32::new(0);
        input
            .iter()
            .map(|&x| cell.fetch_add(u32::from(x), Ordering::Relaxed))
            .collect()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
