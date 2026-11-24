// rust-track: handwritten task-1596
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1596;

pub fn feature_scaling_threshold_crossing(input: &[u8], limit: u32) -> Option<(usize, u32)> {
    let observations: Vec<u32> = {
        let stride = if cfg!(feature = "wide") { 2u32 } else { 1u32 };
        input.iter().map(|&b| u32::from(b) * stride).collect()
    };
    observations
        .into_iter()
        .enumerate()
        .find(|&(_, value)| value >= limit)
}
