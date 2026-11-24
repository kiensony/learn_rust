// rust-track: handwritten task-1413
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1413;

pub fn elapsed_seconds_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        use std::time::Duration;
        input
            .iter()
            .map(|&ticks| Duration::from_millis(u64::from(ticks) * 17).as_secs() as u32)
            .collect()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
