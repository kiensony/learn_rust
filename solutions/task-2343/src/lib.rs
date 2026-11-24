// rust-track: handwritten task-2343
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2343;

pub fn baseline_drift_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        let base = input.first().copied().unwrap_or(0);
        input.iter().map(|&x| u32::from(x.abs_diff(base))).collect()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
