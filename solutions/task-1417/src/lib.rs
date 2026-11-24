// rust-track: handwritten task-1417
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1417;

pub fn elapsed_seconds_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples = observe_elapsed_seconds(input);
    samples.sort_unstable();
    samples.dedup();
    samples
}

fn observe_elapsed_seconds(input: &[u8]) -> Vec<u32> {
    use std::time::Duration;
    input
        .iter()
        .map(|&ticks| Duration::from_millis(u64::from(ticks) * 17).as_secs() as u32)
        .collect()
}
