#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn clamp_property_reverse(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .iter()
            .map(|&x| 1 - u32::from(x.clamp(4, 16) != x))
            .collect()
    };
    samples.into_iter().rev().collect()
}
