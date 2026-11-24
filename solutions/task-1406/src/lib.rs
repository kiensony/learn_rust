// rust-track: handwritten task-1406
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1406;

pub fn millisecond_remainder_descending_steps(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        use std::time::Duration;
        input
            .iter()
            .map(|&ticks| Duration::from_millis(u64::from(ticks) * 17).subsec_millis())
            .collect()
    };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
