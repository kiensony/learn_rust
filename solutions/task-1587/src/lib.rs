// rust-track: handwritten task-1587
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1587;

pub fn feature_lanes_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        let stride = if cfg!(feature = "wide") { 16u32 } else { 1u32 };
        input.iter().map(|&b| u32::from(b) * stride).collect()
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
