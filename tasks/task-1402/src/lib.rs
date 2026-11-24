#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn millisecond_remainder_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        use std::time::Duration;
        input
            .iter()
            .map(|&ticks| Duration::from_millis(u64::from(ticks) * 17).subsec_millis())
            .collect()
    };
    let _ = samples;
    0
}
