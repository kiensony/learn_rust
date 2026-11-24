#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn millisecond_remainder_steps(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        use std::time::Duration;
        input
            .iter()
            .map(|&ticks| Duration::from_millis(u64::from(ticks) * 17).subsec_millis())
            .collect()
    };
    let _ = samples;
    Vec::new()
}
