// rust-track: handwritten task-2366
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2366;

pub fn single_pass_prefix_minimum_descending_steps(input: &[u8]) -> u64 {
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
    samples.windows(2).filter(|p| p[1] < p[0]).count() as u64
}
