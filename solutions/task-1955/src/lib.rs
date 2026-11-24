// rust-track: handwritten task-1955
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1955;

pub fn atomic_running_total_changes(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        use std::sync::atomic::{AtomicU32, Ordering};
        let cell = AtomicU32::new(0);
        input
            .iter()
            .map(|&x| cell.fetch_add(u32::from(x), Ordering::Relaxed))
            .collect()
    };
    samples.dedup();
    samples
}
