// rust-track: handwritten task-1576
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1576;

pub fn feature_packing_descending_steps(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let stride = if cfg!(feature = "wide") { 8u32 } else { 1u32 };
        input.iter().map(|&b| u32::from(b) * stride).collect()
    };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
