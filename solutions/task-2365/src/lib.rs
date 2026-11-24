// rust-track: handwritten task-2365
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2365;

pub fn single_pass_prefix_minimum_changes(input: &[u8]) -> Vec<u32> {
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
    samples.dedup();
    samples
}
