// rust-track: handwritten task-1960
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1960;

pub fn atomic_running_total_reverse(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        use std::sync::atomic::{AtomicU32, Ordering};
        let cell = AtomicU32::new(0);
        input
            .iter()
            .map(|&x| cell.fetch_add(u32::from(x), Ordering::Relaxed))
            .collect()
    };
    samples.into_iter().rev().collect()
}
