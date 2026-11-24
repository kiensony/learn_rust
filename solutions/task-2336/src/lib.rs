// rust-track: handwritten task-2336
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2336;

pub fn batch_variation_descending_steps(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        input
            .chunks(4)
            .map(|c| u32::from(c.iter().max().unwrap() - c.iter().min().unwrap()))
            .collect()
    };
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
