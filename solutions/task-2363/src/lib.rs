// rust-track: handwritten task-2363
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2363;

pub fn single_pass_prefix_minimum_bounds(input: &[u8]) -> Option<(u32, u32)> {
    let samples: Vec<u32> = {
        let mut m = u8::MAX;
        input
            .iter()
            .map(|&x| {
                m = m.min(x);
                u32::from(m)
            })
            .collect()
    };
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
