#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn single_pass_prefix_maximum_distinct(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
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
    samples.sort_unstable();
    samples.dedup();
    samples
}
