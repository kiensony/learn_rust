#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn timer_windows_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        use std::time::Duration;
        input
            .iter()
            .map(|&ticks| {
                Duration::from_millis(u64::from(ticks) * 10)
                    .as_millis()
                    .div_ceil(100) as u32
            })
            .collect()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
