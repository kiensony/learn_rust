// rust-track: handwritten task-2341
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2341;

pub fn baseline_drift_trace(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let base = input.first().copied().unwrap_or(0);
        input.iter().map(|&x| u32::from(x.abs_diff(base))).collect()
    };
    samples
}
