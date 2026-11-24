#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn saturation_property_steps(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            input
                .iter()
                .map(|&x| u32::from(x.saturating_add(1) == x))
                .collect()
        };
        samples.dedup();
        samples
    };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
