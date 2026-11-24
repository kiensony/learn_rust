// rust-track: handwritten task-2355
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2355;

pub fn single_pass_prefix_maximum_changes(input: &[u8]) -> Vec<u32> {
    let mut samples: Vec<u32> = {
        let mut m = 0;
        input
            .iter()
            .map(|&x| {
                m = m.max(x);
                u32::from(m)
            })
            .collect()
    };
    samples.dedup();
    samples
}
