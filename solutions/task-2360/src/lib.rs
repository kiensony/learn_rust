// rust-track: handwritten task-2360
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2360;

pub fn single_pass_prefix_maximum_reverse(input: &[u8]) -> Vec<u32> {
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
    samples.into_iter().rev().collect()
}
