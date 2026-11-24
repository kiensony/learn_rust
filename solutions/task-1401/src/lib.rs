// rust-track: handwritten task-1401
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1401;

pub fn millisecond_remainder_trace(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        use std::time::Duration;
        input
            .iter()
            .map(|&ticks| Duration::from_millis(u64::from(ticks) * 17).subsec_millis())
            .collect()
    };
    samples
}
