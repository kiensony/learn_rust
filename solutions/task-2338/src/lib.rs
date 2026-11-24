// rust-track: handwritten task-2338
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2338;

pub fn batch_variation_steps(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .chunks(4)
            .map(|c| u32::from(c.iter().max().unwrap() - c.iter().min().unwrap()))
            .collect()
    };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
