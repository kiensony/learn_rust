// rust-track: handwritten task-1430
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1430;

pub fn remaining_budget_reverse(input: &[u8]) -> Vec<u32> {
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
    samples.into_iter().rev().collect()
}
