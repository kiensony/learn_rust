// rust-track: handwritten task-2333
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2333;

pub fn batch_variation_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        input
            .chunks(4)
            .map(|c| u32::from(c.iter().max().unwrap() - c.iter().min().unwrap()))
            .collect()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
