// rust-track: handwritten task-1405
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1405;

pub fn millisecond_remainder_changes(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        use std::time::Duration;
        input
            .iter()
            .map(|&ticks| Duration::from_millis(u64::from(ticks) * 17).subsec_millis())
            .collect()
    };
    samples.dedup();
    samples
}
