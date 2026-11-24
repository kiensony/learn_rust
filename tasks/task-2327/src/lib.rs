#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn batch_medians_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
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
        samples.truncate(samples.len().saturating_sub(1));
        samples
    };
    samples.sort_unstable();
    samples.dedup();
    samples
}
