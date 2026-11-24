#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

pub fn single_pass_prefix_minimum_steps(input: &[u8]) -> Vec<u32> {
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
        samples.reverse();
        samples
    };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
