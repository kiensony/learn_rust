// rust-track: handwritten task-1432
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1432;

pub fn timer_windows_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        use std::time::Duration;
        input
            .iter()
            .map(|&ticks| {
                Duration::from_millis(u64::from(ticks) * 17)
                    .as_millis()
                    .div_ceil(100) as u32
            })
            .collect()
    };
    samples.into_iter().map(u64::from).sum()
}
