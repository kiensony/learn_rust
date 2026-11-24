#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn single_pass_prefix_minimum_total(input: &[u8]) -> u64 {
    let samples: Vec<u32> = {
        let mut samples: Vec<u32> = {
            let mut m = u8::MAX;
            input
                .iter()
                .map(|&x| {
                    m = m.min(x);
                    u32::from(m)
                })
                .collect()
        };
        if let Some(first) = samples.first().copied() {
            samples = vec![first];
        }
        samples
    };
    samples.into_iter().map(u64::from).sum()
}
