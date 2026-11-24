// rust-track: handwritten task-1407
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1407;

pub fn millisecond_remainder_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples = observe_millisecond_remainder(input);
    samples.sort_unstable();
    samples.dedup();
    samples
}

fn observe_millisecond_remainder(input: &[u8]) -> Vec<u32> {
    use std::time::Duration;
    input
        .iter()
        .map(|&ticks| Duration::from_millis(u64::from(ticks) * 17).subsec_millis())
        .collect()
}
