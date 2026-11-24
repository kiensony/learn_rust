// rust-track: handwritten task-1952
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1952;

pub fn atomic_running_total_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        use std::sync::atomic::{AtomicU32, Ordering};
        let cell = AtomicU32::new(0);
        input
            .iter()
            .map(|&x| cell.fetch_add(u32::from(x), Ordering::Relaxed))
            .collect()
    };
    samples.into_iter().map(u64::from).sum()
}
