// rust-track: handwritten task-1991
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1991;

pub fn atomic_exchange_trace(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        use std::sync::atomic::{AtomicU32, Ordering};
        let cell = AtomicU32::new(0);
        input
            .iter()
            .map(|&x| cell.swap(u32::from(x), Ordering::Relaxed))
            .collect()
    };
    samples
}
