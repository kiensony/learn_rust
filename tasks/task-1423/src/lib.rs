#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn remaining_budget_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        use std::time::Duration;
        input
            .iter()
            .map(|&ticks| {
                Duration::from_millis(u64::from(ticks) * 17)
                    .saturating_sub(Duration::from_millis(50))
                    .as_millis() as u32
            })
            .collect()
    };
    let _ = samples;
    None
}
