// rust-track: handwritten task-1567
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 1567;

pub fn feature_padding_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples = observe_feature_padding(input);
    samples.sort_unstable();
    samples.dedup();
    samples
}

fn observe_feature_padding(input: &[u8]) -> Vec<u32> {
    let stride = if cfg!(feature = "wide") { 4u32 } else { 1u32 };
    input.iter().map(|&b| u32::from(b) * stride).collect()
}
