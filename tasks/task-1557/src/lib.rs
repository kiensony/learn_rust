#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn feature_scaling_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            let stride = if cfg!(feature = "wide") { 2u32 } else { 1u32 };
            input.iter().map(|&b| u32::from(b) * stride).collect()
        };
        samples.truncate(samples.len().saturating_sub(1));
        samples
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
