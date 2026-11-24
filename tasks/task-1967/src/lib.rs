#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn atomic_bit_union_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            use std::sync::atomic::{AtomicU32, Ordering};
            let cell = AtomicU32::new(0);
            input
                .iter()
                .map(|&x| cell.fetch_or(u32::from(x), Ordering::Relaxed))
                .collect()
        };
        samples.truncate(samples.len().saturating_sub(1));
        samples
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
