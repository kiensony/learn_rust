// rust-track: handwritten task-1429
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1429;

pub fn remaining_budget_longest_run(input: &[u8]) -> u64 {
    let samples = observe_remaining_budget(input);
    let (mut previous, mut run, mut longest) = (None, 0u64, 0);
    for x in samples {
        run = if previous == Some(x) { run + 1 } else { 1 };
        longest = longest.max(run);
        previous = Some(x);
    }
    longest
}

fn observe_remaining_budget(input: &[u8]) -> Vec<u32> {
    use std::time::Duration;
    input
        .iter()
        .map(|&ticks| {
            Duration::from_millis(u64::from(ticks) * 17)
                .saturating_sub(Duration::from_millis(50))
                .as_millis() as u32
        })
        .collect()
}
