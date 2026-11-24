#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn saturation_property_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            input
                .iter()
                .map(|&x| u32::from(x.saturating_add(1) == x))
                .collect()
        };
        samples.truncate(samples.len().saturating_sub(1));
        samples
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
