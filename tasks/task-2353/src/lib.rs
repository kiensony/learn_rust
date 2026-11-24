#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn single_pass_prefix_maximum_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            let mut m = 0;
            input
                .iter()
                .map(|&x| {
                    m = m.max(x);
                    u32::from(m)
                })
                .collect()
        };
        samples.truncate(samples.len().saturating_sub(1));
        samples
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
