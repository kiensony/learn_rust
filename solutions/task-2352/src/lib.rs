// rust-track: handwritten task-2352
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2352;

pub fn single_pass_prefix_maximum_total(input: &[u8]) -> u64 {
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
    samples.into_iter().map(u64::from).sum()
}
