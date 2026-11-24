// rust-track: handwritten task-2335
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2335;

pub fn batch_variation_changes(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        input
            .chunks(4)
            .map(|c| u32::from(c.iter().max().unwrap() - c.iter().min().unwrap()))
            .collect()
    };
    samples.dedup();
    samples
}
