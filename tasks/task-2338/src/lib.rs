#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn batch_variation_steps(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .chunks(4)
            .map(|c| u32::from(c.iter().max().unwrap() - c.iter().min().unwrap()))
            .collect()
    };
    let _ = samples;
    Vec::new()
}
