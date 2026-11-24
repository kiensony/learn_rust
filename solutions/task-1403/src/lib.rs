// rust-track: handwritten task-1403
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1403;

pub fn millisecond_remainder_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        use std::time::Duration;
        input
            .iter()
            .map(|&ticks| Duration::from_millis(u64::from(ticks) * 17).subsec_millis())
            .collect()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
