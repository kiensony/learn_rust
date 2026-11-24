#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0;

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
    let _ = samples;
    0
}
