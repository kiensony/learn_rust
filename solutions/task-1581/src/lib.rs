// rust-track: handwritten task-1581
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1581;

pub fn feature_lanes_trace(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let stride = if cfg!(feature = "wide") { 16u32 } else { 1u32 };
        input.iter().map(|&b| u32::from(b) * stride).collect()
    };
    samples
}
