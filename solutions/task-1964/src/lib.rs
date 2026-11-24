// rust-track: handwritten task-1964
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1964;

pub fn atomic_bit_union_peaks(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        use std::sync::atomic::{AtomicU32, Ordering};
        let cell = AtomicU32::new(0);
        input
            .iter()
            .map(|&x| cell.fetch_or(u32::from(x), Ordering::Relaxed))
            .collect()
    };
    let mut peak = None;
    samples
        .into_iter()
        .filter(|&x| {
            if peak.is_none_or(|p| x > p) {
                peak = Some(x);
                true
            } else {
                false
            }
        })
        .collect()
}
