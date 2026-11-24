// rust-track: handwritten task-2396
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2396;

pub fn single_pass_prefix_maximum_threshold_crossing(
    input: &[u8],
    limit: u32,
) -> Option<(usize, u32)> {
    let observations: Vec<u32> = {
        let mut m = 0;
        input
            .iter()
            .map(|&x| {
                m = m.max(x);
                u32::from(m)
            })
            .collect()
    };
    observations
        .into_iter()
        .enumerate()
        .find(|&(_, value)| value >= limit)
}
