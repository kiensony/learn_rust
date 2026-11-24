// rust-track: handwritten task-2327
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2327;

pub fn batch_medians_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        input
            .chunks(3)
            .map(|c| {
                let mut v = c.to_vec();
                v.sort_unstable();
                u32::from(v[(v.len() - 1) / 2])
            })
            .collect()
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
