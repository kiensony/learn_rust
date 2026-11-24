// rust-track: handwritten task-1570
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1570;

pub fn feature_padding_reverse(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let stride = if cfg!(feature = "wide") { 4u32 } else { 1u32 };
        input.iter().map(|&b| u32::from(b) * stride).collect()
    };
    samples.into_iter().rev().collect()
}
