#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn batch_variation_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            input
                .chunks(4)
                .map(|c| u32::from(c.iter().max().unwrap() - c.iter().min().unwrap()))
                .collect()
        };
        samples.truncate(samples.len().saturating_sub(1));
        samples
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
