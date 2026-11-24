// rust-track: handwritten task-2328
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2328;

pub fn batch_medians_steps(input: &[u8]) -> Vec<u32> {
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
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
