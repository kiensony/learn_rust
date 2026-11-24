// rust-track: handwritten task-2388
#![forbid(unsafe_code)]

pub const TRACK_PROOF: u32 = 0x52540000 | 2388;

pub fn rolling_parity_steps(input: &[u8]) -> Vec<u32> {
    let samples: Vec<u32> = {
        let mut p = 0;
        input
            .iter()
            .map(|&x| {
                p ^= x;
                u32::from(p)
            })
            .collect()
    };
    samples.windows(2).map(|p| p[0].abs_diff(p[1])).collect()
}
