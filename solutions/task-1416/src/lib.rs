// rust-track: handwritten task-1416
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1416;

pub fn elapsed_seconds_descending_steps(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        use std::time::Duration;
        input
            .iter()
            .map(|&ticks| Duration::from_millis(u64::from(ticks) * 17).as_secs() as u32)
            .collect()
    };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
