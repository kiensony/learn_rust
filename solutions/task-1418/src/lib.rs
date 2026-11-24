// rust-track: handwritten task-1418
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1418;

pub fn elapsed_seconds_steps(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        use std::time::Duration;
        input
            .iter()
            .map(|&ticks| Duration::from_millis(u64::from(ticks) * 17).as_secs() as u32)
            .collect()
    };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
