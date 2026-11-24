// rust-track: handwritten task-1975
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1975;

pub fn atomic_toggle_changes(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        use std::sync::atomic::{AtomicU32, Ordering};
        let cell = AtomicU32::new(0);
        input
            .iter()
            .map(|&x| cell.fetch_xor(u32::from(x), Ordering::Relaxed))
            .collect()
    };
    samples.dedup();
    samples
}
