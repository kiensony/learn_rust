// rust-track: handwritten task-1592
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1592;

pub fn feature_pages_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let stride = if cfg!(feature = "wide") { 32u32 } else { 1u32 };
        input.iter().map(|&b| u32::from(b) * stride).collect()
    };
    samples.into_iter().map(u64::from).sum()
}
