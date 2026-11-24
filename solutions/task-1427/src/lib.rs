// rust-track: handwritten task-1427
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1427;

pub fn remaining_budget_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
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
    samples.sort_unstable();
    samples.dedup();
    samples
}
