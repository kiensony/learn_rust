// rust-track: handwritten task-1970
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1970;

pub fn atomic_bit_union_reverse(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        use std::sync::atomic::{AtomicU32, Ordering};
        let cell = AtomicU32::new(0);
        input
            .iter()
            .map(|&x| cell.fetch_or(u32::from(x), Ordering::Relaxed))
            .collect()
    };
    samples.into_iter().rev().collect()
}
