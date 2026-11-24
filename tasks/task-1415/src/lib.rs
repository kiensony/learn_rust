#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn elapsed_seconds_changes(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        use std::time::Duration;
        input
            .iter()
            .map(|&ticks| Duration::from_millis(u64::from(ticks) * 10).as_secs() as u32)
            .collect()
    };
    samples.dedup();
    samples
}
