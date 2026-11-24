// rust-track: handwritten task-2353
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2353;

pub fn single_pass_prefix_maximum_bounds(input: &[u8]) -> Option<(u32, u32)> {
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
    samples
        .iter()
        .min()
        .copied()
        .zip(samples.iter().max().copied())
}
