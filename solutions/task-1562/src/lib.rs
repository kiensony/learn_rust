// rust-track: handwritten task-1562
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1562;

pub fn feature_padding_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let stride = if cfg!(feature = "wide") { 4u32 } else { 1u32 };
        input.iter().map(|&b| u32::from(b) * stride).collect()
    };
    samples.into_iter().map(u64::from).sum()
}
