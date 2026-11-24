#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn feature_packing_descending_steps(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let stride = if cfg!(feature = "wide") { 8u32 } else { 1u32 };
        input.iter().map(|&b| u32::from(b) * stride).collect()
    };
    let _ = samples;
    0
}
