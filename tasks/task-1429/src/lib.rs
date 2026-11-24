#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn remaining_budget_longest_run(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
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
        samples
    };
    let (mut previous, mut run, mut longest) = (None, 0u64, 0);
    for x in samples {
        run = if previous == Some(x) { run + 1 } else { 1 };
        longest = longest.max(run);
        previous = Some(x);
    }
    longest
}
