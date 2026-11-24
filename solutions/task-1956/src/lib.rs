// rust-track: handwritten task-1956
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1956;

pub fn atomic_running_total_descending_steps(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        use std::sync::atomic::{AtomicU32, Ordering};
        let cell = AtomicU32::new(0);
        input
            .iter()
            .map(|&x| cell.fetch_add(u32::from(x), Ordering::Relaxed))
            .collect()
    };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
