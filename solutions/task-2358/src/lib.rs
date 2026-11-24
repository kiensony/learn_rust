// rust-track: handwritten task-2358
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2358;

pub fn single_pass_prefix_maximum_steps(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut m = 0;
        input
            .iter()
            .map(|&x| {
                m = m.max(x);
                u32::from(m)
            })
            .collect()
    };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
