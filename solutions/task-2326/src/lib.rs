// rust-track: handwritten task-2326
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2326;

pub fn batch_medians_descending_steps(input: &[u8]) -> u64 {
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
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
