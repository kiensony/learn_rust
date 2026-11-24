// rust-track: handwritten task-2330
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2330;

pub fn batch_medians_reverse(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        input
            .chunks(3)
            .map(|c| {
                let mut v = c.to_vec();
                v.sort_unstable();
                u32::from(v[(v.len() - 1) / 2])
            })
            .collect()
    };
    samples.into_iter().rev().collect()
}
