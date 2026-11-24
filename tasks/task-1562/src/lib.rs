#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn feature_padding_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let stride = if cfg!(feature = "wide") { 4u32 } else { 2u32 };
        input.iter().map(|&b| u32::from(b) * stride).collect()
    };
    samples.into_iter().map(u64::from).sum()
}
