// rust-track: handwritten task-1568
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1568;

pub fn feature_padding_steps(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let stride = if cfg!(feature = "wide") { 4u32 } else { 1u32 };
        input.iter().map(|&b| u32::from(b) * stride).collect()
    };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
