#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn deadline_slack_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        use std::time::Duration;
        input
            .iter()
            .map(|&ticks| {
                Duration::from_millis(u64::from(ticks) * 10)
                    .abs_diff(Duration::from_millis(250))
                    .as_millis() as u32
            })
            .collect()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
