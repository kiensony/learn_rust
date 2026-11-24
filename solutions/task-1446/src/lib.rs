// rust-track: handwritten task-1446
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1446;

pub fn millisecond_remainder_threshold_crossing(input: &[u8], limit: u32) -> Option<(usize, u32)> {
    let observations: Vec<u32> = {
        use std::time::Duration;
        input
            .iter()
            .map(|&ticks| Duration::from_millis(u64::from(ticks) * 17).subsec_millis())
            .collect()
    };
    observations
        .into_iter()
        .enumerate()
        .find(|&(_, value)| value >= limit)
}
