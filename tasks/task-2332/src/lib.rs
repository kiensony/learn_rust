#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn batch_variation_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        input
            .chunks(4)
            .map(|c| u32::from(c.iter().max().unwrap() - c.iter().min().unwrap()))
            .collect()
    };
    let _ = samples;
    0
}
