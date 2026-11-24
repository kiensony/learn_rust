#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn atomic_toggle_peaks(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        use std::sync::atomic::{AtomicU32, Ordering};
        let cell = AtomicU32::new(1);
        input
            .iter()
            .map(|&x| cell.fetch_xor(u32::from(x), Ordering::Relaxed))
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
