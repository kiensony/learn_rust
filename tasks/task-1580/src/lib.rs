#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn feature_packing_reverse(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let stride = if cfg!(feature = "wide") { 8u32 } else { 2u32 };
        input.iter().map(|&b| u32::from(b) * stride).collect()
    };
    samples.into_iter().rev().collect()
}
