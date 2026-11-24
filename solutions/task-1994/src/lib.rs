// rust-track: handwritten task-1994
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1994;

pub fn atomic_exchange_peaks(input: &[u8]) -> Vec<u32> {
    let samples = observe_atomic_exchange(input);
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

fn observe_atomic_exchange(input: &[u8]) -> Vec<u32> {
    use std::sync::atomic::{AtomicU32, Ordering};
    let cell = AtomicU32::new(0);
    input
        .iter()
        .map(|&x| cell.swap(u32::from(x), Ordering::Relaxed))
        .collect()
}
