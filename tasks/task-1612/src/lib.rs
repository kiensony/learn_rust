#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn saturation_property_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        input
            .iter()
            .map(|&x| 1 - u32::from(x.saturating_add(1) == x))
            .collect()
    };
    samples.into_iter().map(u64::from).sum()
}
