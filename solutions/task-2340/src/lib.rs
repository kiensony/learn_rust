// rust-track: handwritten task-2340
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2340;

pub fn batch_variation_reverse(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .chunks(4)
            .map(|c| u32::from(c.iter().max().unwrap() - c.iter().min().unwrap()))
            .collect()
    };
    samples.into_iter().rev().collect()
}
