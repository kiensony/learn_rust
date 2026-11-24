// rust-track: handwritten task-1445
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1445;

pub fn deadline_slack_changes(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        use std::time::Duration;
        input
            .iter()
            .map(|&ticks| {
                Duration::from_millis(u64::from(ticks) * 17)
                    .abs_diff(Duration::from_millis(250))
                    .as_millis() as u32
            })
            .collect()
    };
    samples.dedup();
    samples
}
