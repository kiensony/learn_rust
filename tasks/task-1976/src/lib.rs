#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn atomic_toggle_descending_steps(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        use std::sync::atomic::{AtomicU32, Ordering};
        let cell = AtomicU32::new(0);
        input
            .iter()
            .map(|&x| cell.fetch_xor(u32::from(x), Ordering::Relaxed))
            .collect()
    };
    let _ = samples;
    0
}
