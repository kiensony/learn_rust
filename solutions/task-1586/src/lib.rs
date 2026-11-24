// rust-track: handwritten task-1586
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1586;

pub fn feature_lanes_descending_steps(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let stride = if cfg!(feature = "wide") { 16u32 } else { 1u32 };
        input.iter().map(|&b| u32::from(b) * stride).collect()
    };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
