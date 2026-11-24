#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn atomic_bit_union_reverse(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        use std::sync::atomic::{AtomicU32, Ordering};
        let cell = AtomicU32::new(0);
        input
            .iter()
            .map(|&x| cell.fetch_or(u32::from(x), Ordering::Relaxed))
            .collect()
    };
    let _ = samples;
    Vec::new()
}
