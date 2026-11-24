// rust-track: handwritten task-1988
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1988;

pub fn atomic_record_high_steps(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        use std::sync::atomic::{AtomicU32, Ordering};
        let cell = AtomicU32::new(0);
        input
            .iter()
            .map(|&x| cell.fetch_max(u32::from(x), Ordering::Relaxed))
            .collect()
    };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
